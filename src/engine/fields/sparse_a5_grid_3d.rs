//! Sparse 3D object grid keyed by [A5](https://github.com/jonititan/bevy_a5)
//! pentagonal cells stratified into altitude layers.
//!
//! [`SparseA5Grid`](super::sparse_a5_grid::SparseA5Grid) covers the planet
//! surface as a 2D tessellation. `SparseA5Grid3D` extends that with a
//! caller-defined stack of altitude layers — useful when agents live at
//! distinct heights (sea level, troposphere, satellites; subsurface,
//! seabed, surface, atmosphere; etc.).
//!
//! # Layer semantics
//!
//! Layers are defined by an optional ascending [`Vec<f64>`] of altitudes
//! that doubles as the simulation's vertical extent:
//!
//! - `boundaries[0]` is the floor of the sim.
//! - `boundaries.last()` is the ceiling of the sim.
//! - Every value in between is an internal layer boundary.
//! - `N` boundaries (`N >= 2`) define `N − 1` layers; layer `i` covers
//!   altitudes in `[boundaries[i], boundaries[i + 1])`, with the top
//!   layer inclusive of the ceiling.
//! - Altitudes outside `[boundaries[0], boundaries.last()]` are out of
//!   domain and produce `None` from the binning helpers.
//!
//! Pass `None` (or a vec with fewer than two entries) for a single-layer
//! grid that accepts any altitude.
//!
//! # Storage backend
//!
//! Same backend rule as [`SparseA5Grid`]: `parallel` / `visualization*` →
//! `DBDashMap` (thread-safe), otherwise `RefCell<HashMap<...>>`.
//!
//! Available under the `gis` cargo feature.

use crate::engine::fields::{field::Field, grid_option::GridOption};

use bevy_a5::prelude::GeoCell;
use bevy_a5::{query, WORLD_CELL};

use cfg_if::cfg_if;
use rand::Rng;
use std::hash::Hash;

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))] {
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use hashbrown::HashMap;
        use std::cell::RefCell;
    }
}

/// 3D location: an A5 cell plus a discrete altitude layer index.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct A5Cell3D {
    pub cell: GeoCell,
    pub layer: usize,
}

impl A5Cell3D {
    pub fn new(cell: GeoCell, layer: usize) -> Self {
        Self { cell, layer }
    }
}

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))] {
        // -----------------------------------------------------------------
        // Parallel / visualization branch — DBDashMap-backed, thread-safe.
        // -----------------------------------------------------------------

        pub struct SparseA5Grid3D<O: Eq + Hash + Clone + Copy> {
            pub obj2loc: DBDashMap<O, A5Cell3D>,
            pub loc2objs: DBDashMap<A5Cell3D, Vec<O>>,
            pub root: GeoCell,
            pub resolution: i32,
            pub layer_boundaries: Option<Vec<f64>>,
        }

        impl<O: Eq + Hash + Clone + Copy> SparseA5Grid3D<O> {
            pub fn new(resolution: i32, layer_boundaries: Option<Vec<f64>>) -> Self {
                Self::new_with_root(WORLD_CELL.into(), resolution, layer_boundaries)
            }

            pub fn new_with_root(
                root: GeoCell,
                resolution: i32,
                layer_boundaries: Option<Vec<f64>>,
            ) -> Self {
                let layer_boundaries = normalise_layer_boundaries(layer_boundaries);
                SparseA5Grid3D {
                    obj2loc: DBDashMap::new(),
                    loc2objs: DBDashMap::new(),
                    root,
                    resolution,
                    layer_boundaries,
                }
            }

            pub fn apply_to_all_values<F>(&self, closure: F, _option: GridOption)
            where
                F: Fn(&A5Cell3D, &O) -> Option<O>,
            {
                self.obj2loc.apply_to_all_keys(closure);
            }

            pub fn set_object_location(&self, object: O, loc: &A5Cell3D) {
                match self.loc2objs.get_write(loc) {
                    Some(mut vec) => {
                        if !vec.is_empty() {
                            vec.retain(|&x| x != object);
                        }
                        vec.push(object);
                    }
                    None => {
                        self.loc2objs.insert(*loc, vec![object]);
                    }
                }
                self.obj2loc.insert(object, *loc);
            }

            pub fn remove_object_location(&self, object: O, loc: &A5Cell3D) {
                let now_empty = if let Some(mut vec) = self.loc2objs.get_write(loc) {
                    vec.retain(|&x| x != object);
                    vec.is_empty()
                } else {
                    false
                };
                if now_empty {
                    self.loc2objs.remove(loc);
                }
                self.obj2loc.remove(&object);
            }

            pub fn remove_object(&self, object: &O) {
                if let Some(loc_ref) = self.obj2loc.get_read(object) {
                    let loc = *loc_ref;
                    if let Some(mut vec) = self.loc2objs.get_write(&loc) {
                        vec.retain(|x| x != object);
                    }
                }
                self.obj2loc.remove(object);
            }

            pub fn get(&self, object: &O) -> Option<O> {
                self.obj2loc.get_key_value(object).map(|(k, _)| *k)
            }

            pub fn get_unbuffered(&self, object: &O) -> Option<O> {
                self.obj2loc.get_write(object).map(|_| *object)
            }

            pub fn get_location(&self, object: &O) -> Option<A5Cell3D> {
                self.obj2loc.get_read(object).copied()
            }

            pub fn get_location_unbuffered(&self, object: &O) -> Option<A5Cell3D> {
                self.obj2loc.get_write(object).map(|loc_ref| *loc_ref)
            }

            pub fn get_objects(&self, loc: &A5Cell3D) -> Option<Vec<O>> {
                match self.loc2objs.get_read(loc) {
                    Some(vec) if !vec.is_empty() => Some(vec.clone()),
                    _ => None,
                }
            }

            pub fn get_objects_unbuffered(&self, loc: &A5Cell3D) -> Option<Vec<O>> {
                match self.loc2objs.get_write(loc) {
                    Some(vec) if !vec.is_empty() => Some(vec.clone()),
                    _ => None,
                }
            }

            pub fn num_objects(&self) -> usize {
                let mut total = 0;
                for shard in self.loc2objs.r_shards.iter() {
                    for bag in shard.values() {
                        total += bag.len();
                    }
                }
                total
            }

            pub fn num_objects_at_location(&self, loc: &A5Cell3D) -> usize {
                self.loc2objs
                    .get_read(loc)
                    .map(|bag| bag.len())
                    .unwrap_or(0)
            }

            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(&A5Cell3D, &O),
            {
                for shard in self.loc2objs.r_shards.iter() {
                    for (key, bag) in shard.iter() {
                        for obj in bag {
                            closure(key, obj);
                        }
                    }
                }
            }

            pub fn iter_objects_unbuffered<F>(&self, closure: F)
            where
                F: Fn(&A5Cell3D, &O),
            {
                for shard_mutex in self.loc2objs.shards.iter() {
                    let shard = shard_mutex.lock().expect("lock loc2objs shard");
                    for (key, bag) in shard.iter() {
                        for obj in bag {
                            closure(key, obj);
                        }
                    }
                }
            }

            pub fn get_empty_bags(&self) -> Vec<A5Cell3D> {
                let mut empty = Vec::new();
                if let Some(locs) = self.all_locations() {
                    for loc in locs {
                        match self.loc2objs.get_read(&loc) {
                            Some(bag) if !bag.is_empty() => {}
                            _ => empty.push(loc),
                        }
                    }
                }
                empty
            }

            pub fn get_random_empty_bag(&self) -> Option<A5Cell3D> {
                let empty = self.get_empty_bags();
                if empty.is_empty() {
                    return None;
                }
                let mut rng = rand::rng();
                Some(empty[rng.random_range(0..empty.len())])
            }

            fn collect_objects(&self, locs: &[A5Cell3D]) -> Vec<O> {
                let mut out = Vec::new();
                for loc in locs {
                    if let Some(bag) = self.loc2objs.get_read(loc) {
                        out.extend(bag.iter().copied());
                    }
                }
                out
            }
        }

        impl<O: Eq + Hash + Clone + Copy> Field for SparseA5Grid3D<O> {
            fn lazy_update(&mut self) {
                self.obj2loc.lazy_update();
                self.loc2objs.lazy_update();
            }

            fn update(&mut self) {
                self.obj2loc.update();
                self.loc2objs.update();
            }
        }
    } else {
        // -----------------------------------------------------------------
        // Single-threaded branch — RefCell + HashMap, double-buffered.
        // -----------------------------------------------------------------

        pub struct SparseA5Grid3D<O: Eq + Hash + Clone + Copy> {
            pub locs: Vec<RefCell<HashMap<A5Cell3D, Vec<O>>>>,
            read: usize,
            write: usize,
            pub root: GeoCell,
            pub resolution: i32,
            pub layer_boundaries: Option<Vec<f64>>,
        }

        impl<O: Eq + Hash + Clone + Copy> SparseA5Grid3D<O> {
            pub fn new(resolution: i32, layer_boundaries: Option<Vec<f64>>) -> Self {
                Self::new_with_root(WORLD_CELL.into(), resolution, layer_boundaries)
            }

            pub fn new_with_root(
                root: GeoCell,
                resolution: i32,
                layer_boundaries: Option<Vec<f64>>,
            ) -> Self {
                let layer_boundaries = normalise_layer_boundaries(layer_boundaries);
                SparseA5Grid3D {
                    locs: vec![
                        RefCell::new(HashMap::new()),
                        RefCell::new(HashMap::new()),
                    ],
                    read: 0,
                    write: 1,
                    root,
                    resolution,
                    layer_boundaries,
                }
            }

            pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
            where
                F: Fn(&A5Cell3D, &O) -> Option<O>,
            {
                match option {
                    GridOption::READ => {
                        let mut rlocs = self.locs[self.read].borrow_mut();
                        for (key, value) in rlocs.iter_mut() {
                            for obj in value {
                                *obj = closure(key, obj).expect("error on closure");
                            }
                        }
                    }
                    GridOption::WRITE => {
                        let mut locs = self.locs[self.write].borrow_mut();
                        for (key, value) in locs.iter_mut() {
                            for obj in value {
                                *obj = closure(key, obj).expect("error on closure");
                            }
                        }
                    }
                    GridOption::READWRITE => {
                        let rlocs = self.locs[self.read].borrow();
                        let mut locs = self.locs[self.write].borrow_mut();
                        for (key, value) in rlocs.iter() {
                            if let Some(write_value) = locs.get_mut(key) {
                                for obj in write_value {
                                    *obj = closure(key, obj).expect("error on closure");
                                }
                            } else {
                                for obj in value {
                                    let new_bag = vec![closure(key, obj).expect("error on closure")];
                                    locs.insert(*key, new_bag);
                                }
                            }
                        }
                    }
                }
            }

            pub fn get_location(&self, object: &O) -> Option<A5Cell3D> {
                let rlocs = self.locs[self.read].borrow();
                for (key, objs) in rlocs.iter() {
                    for obj in objs {
                        if *obj == *object {
                            return Some(*key);
                        }
                    }
                }
                None
            }

            pub fn get_location_unbuffered(&self, object: &O) -> Option<A5Cell3D> {
                let locs = self.locs[self.write].borrow();
                for (key, objs) in locs.iter() {
                    for obj in objs {
                        if *obj == *object {
                            return Some(*key);
                        }
                    }
                }
                None
            }

            pub fn get(&self, object: &O) -> Option<O> {
                let rlocs = self.locs[self.read].borrow();
                for bag in rlocs.values() {
                    for obj in bag {
                        if *obj == *object {
                            return Some(*obj);
                        }
                    }
                }
                None
            }

            pub fn get_unbuffered(&self, object: &O) -> Option<O> {
                let locs = self.locs[self.write].borrow();
                for bag in locs.values() {
                    for obj in bag {
                        if *obj == *object {
                            return Some(*obj);
                        }
                    }
                }
                None
            }

            pub fn num_objects(&self) -> usize {
                self.locs[self.read]
                    .borrow()
                    .values()
                    .map(|bag| bag.len())
                    .sum()
            }

            pub fn num_objects_at_location(&self, loc: &A5Cell3D) -> usize {
                self.locs[self.read]
                    .borrow()
                    .get(loc)
                    .map(|bag| bag.len())
                    .unwrap_or(0)
            }

            pub fn get_objects(&self, loc: &A5Cell3D) -> Option<Vec<O>> {
                self.locs[self.read].borrow().get(loc).cloned()
            }

            pub fn get_objects_unbuffered(&self, loc: &A5Cell3D) -> Option<Vec<O>> {
                self.locs[self.write].borrow().get(loc).cloned()
            }

            pub fn get_empty_bags(&self) -> Vec<A5Cell3D> {
                let mut empty = Vec::new();
                let rlocs = self.locs[self.read].borrow();
                if let Some(locs) = self.all_locations() {
                    for loc in locs {
                        match rlocs.get(&loc) {
                            Some(bag) if !bag.is_empty() => {}
                            _ => empty.push(loc),
                        }
                    }
                }
                empty
            }

            pub fn get_random_empty_bag(&self) -> Option<A5Cell3D> {
                let empty = self.get_empty_bags();
                if empty.is_empty() {
                    return None;
                }
                let mut rng = rand::rng();
                Some(empty[rng.random_range(0..empty.len())])
            }

            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(&A5Cell3D, &O),
            {
                let rlocs = self.locs[self.read].borrow();
                for (key, bag) in rlocs.iter() {
                    for obj in bag {
                        closure(key, obj);
                    }
                }
            }

            pub fn iter_objects_unbuffered<F>(&self, closure: F)
            where
                F: Fn(&A5Cell3D, &O),
            {
                let locs = self.locs[self.write].borrow();
                for (key, bag) in locs.iter() {
                    for obj in bag {
                        closure(key, obj);
                    }
                }
            }

            pub fn set_object_location(&self, object: O, loc: &A5Cell3D) {
                let mut locs = self.locs[self.write].borrow_mut();
                match locs.get_mut(loc) {
                    Some(bag) => bag.push(object),
                    None => {
                        locs.insert(*loc, vec![object]);
                    }
                }
            }

            pub fn remove_object_location(&self, object: O, loc: &A5Cell3D) {
                let mut locs = self.locs[self.write].borrow_mut();
                if let Some(bag) = locs.get_mut(loc) {
                    bag.retain(|&obj| obj != object);
                    if bag.is_empty() {
                        locs.remove(loc);
                    }
                }
            }

            pub fn remove_object(&self, object: &O) {
                let mut locs = self.locs[self.write].borrow_mut();
                let mut empty_keys: Vec<A5Cell3D> = Vec::new();
                for (key, bag) in locs.iter_mut() {
                    bag.retain(|obj| obj != object);
                    if bag.is_empty() {
                        empty_keys.push(*key);
                    }
                }
                for key in empty_keys {
                    locs.remove(&key);
                }
            }

            fn collect_objects(&self, locs: &[A5Cell3D]) -> Vec<O> {
                let rlocs = self.locs[self.read].borrow();
                let mut out = Vec::new();
                for loc in locs {
                    if let Some(bag) = rlocs.get(loc) {
                        out.extend(bag.iter().copied());
                    }
                }
                out
            }
        }

        impl<O: Eq + Hash + Clone + Copy> Field for SparseA5Grid3D<O> {
            fn lazy_update(&mut self) {
                std::mem::swap(&mut self.read, &mut self.write);
                self.locs[self.write].borrow_mut().clear();
            }

            fn update(&mut self) {
                let mut rlocs = self.locs[self.read].borrow_mut();
                rlocs.clear();
                for (key, value) in self.locs[self.write].borrow().iter() {
                    rlocs.insert(*key, value.clone());
                }
                self.locs[self.write].borrow_mut().clear();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Storage-agnostic shared API.
// ---------------------------------------------------------------------------

impl<O: Eq + Hash + Clone + Copy> SparseA5Grid3D<O> {
    pub fn num_layers(&self) -> usize {
        self.layer_boundaries
            .as_ref()
            .map(|b| b.len() - 1)
            .unwrap_or(1)
    }

    pub fn floor(&self) -> Option<f64> {
        self.layer_boundaries.as_ref().map(|b| b[0])
    }

    pub fn ceiling(&self) -> Option<f64> {
        self.layer_boundaries.as_ref().map(|b| b[b.len() - 1])
    }

    pub fn layer_for_altitude(&self, altitude: f64) -> Option<usize> {
        let Some(bounds) = &self.layer_boundaries else {
            return Some(0);
        };
        let n = bounds.len();
        if altitude < bounds[0] || altitude > bounds[n - 1] {
            return None;
        }
        for i in 0..n - 1 {
            if altitude < bounds[i + 1] {
                return Some(i);
            }
        }
        Some(n - 2) // altitude == ceiling
    }

    /// Bounding-volume predicate: lateral subtree check + valid layer.
    pub fn contains(&self, loc: &A5Cell3D) -> bool {
        if loc.layer >= self.num_layers() {
            return false;
        }
        if loc.cell.resolution() != self.resolution {
            return false;
        }
        if self.root.is_world_cell() {
            return true;
        }
        let root_res = self.root.resolution();
        if root_res > self.resolution {
            return false;
        }
        loc.cell.parent(root_res) == Some(self.root)
    }

    pub fn all_locations(&self) -> Option<Vec<A5Cell3D>> {
        let cells = self.root.children(self.resolution)?;
        let mut out = Vec::with_capacity(cells.len() * self.num_layers());
        for layer in 0..self.num_layers() {
            for cell in &cells {
                out.push(A5Cell3D::new(*cell, layer));
            }
        }
        Some(out)
    }

    pub fn cell_3d_at(&self, longitude: f64, latitude: f64, altitude: f64) -> Option<A5Cell3D> {
        let cell = GeoCell::from_lon_lat(longitude, latitude, self.resolution)?;
        let layer = self.layer_for_altitude(altitude)?;
        Some(A5Cell3D::new(cell, layer))
    }

    // ---------- Cell-only spatial helpers (within a single layer) ----------

    pub fn cell_neighbors(&self, loc: &A5Cell3D) -> Option<Vec<A5Cell3D>> {
        Some(lift_to_layer(query::neighbors(&loc.cell)?, loc.layer))
    }

    pub fn cell_vertex_neighbors(&self, loc: &A5Cell3D) -> Option<Vec<A5Cell3D>> {
        Some(lift_to_layer(
            query::vertex_neighbors(&loc.cell)?,
            loc.layer,
        ))
    }

    pub fn cell_grid_disk(&self, loc: &A5Cell3D, k: usize) -> Option<Vec<A5Cell3D>> {
        Some(lift_to_layer(query::grid_disk(&loc.cell, k)?, loc.layer))
    }

    pub fn cell_spherical_cap(&self, loc: &A5Cell3D, radius_meters: f64) -> Option<Vec<A5Cell3D>> {
        Some(lift_to_layer(
            query::spherical_cap(&loc.cell, radius_meters)?,
            loc.layer,
        ))
    }

    // ---------- Object-returning spatial queries (single-layer, read buffer) ----------

    pub fn get_neighbors(&self, loc: &A5Cell3D) -> Vec<O> {
        match self.cell_neighbors(loc) {
            Some(cells) => self.collect_objects(&cells),
            None => Vec::new(),
        }
    }

    pub fn get_vertex_neighbors(&self, loc: &A5Cell3D) -> Vec<O> {
        match self.cell_vertex_neighbors(loc) {
            Some(cells) => self.collect_objects(&cells),
            None => Vec::new(),
        }
    }

    pub fn get_objects_within_disk(&self, loc: &A5Cell3D, k: usize) -> Vec<O> {
        match self.cell_grid_disk(loc, k) {
            Some(cells) => self.collect_objects(&cells),
            None => Vec::new(),
        }
    }

    pub fn get_neighbors_within_distance(&self, loc: &A5Cell3D, dist_meters: f64) -> Vec<O> {
        if dist_meters <= 0.0 {
            return Vec::new();
        }
        match self.cell_spherical_cap(loc, dist_meters) {
            Some(cells) => self.collect_objects(&cells),
            None => Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Free helpers (used by both backends).
// ---------------------------------------------------------------------------

fn lift_to_layer(cells: Vec<GeoCell>, layer: usize) -> Vec<A5Cell3D> {
    cells.into_iter().map(|c| A5Cell3D::new(c, layer)).collect()
}

/// Normalise `layer_boundaries`: empty / single-entry → `None`, and panic
/// on non-strictly-ascending input.
fn normalise_layer_boundaries(b: Option<Vec<f64>>) -> Option<Vec<f64>> {
    match b {
        Some(v) if v.len() < 2 => None,
        Some(v) => {
            assert!(
                v.windows(2).all(|w| w[0] < w[1]),
                "SparseA5Grid3D layer boundaries must be strictly ascending"
            );
            Some(v)
        }
        None => None,
    }
}
