//! Sparse object grid keyed by [A5](https://github.com/jonititan/bevy_a5)
//! pentagonal cells.
//!
//! This is the spherical / GIS counterpart of [`SparseGrid2D`](super::sparse_object_grid_2d::SparseGrid2D).
//! Whereas `SparseGrid2D` indexes a flat `width × height` rectangle of
//! [`Int2D`](crate::engine::location::Int2D) cells, `SparseA5Grid` indexes
//! the descendants of a root [`GeoCell`] at a fixed A5 resolution. Default
//! root is `WORLD_CELL` (the whole planet); pass a finer root to bound the
//! grid to a region.
//!
//! # Storage backend
//!
//! Mirrors the 2D sparse grid: when any of the `parallel`, `visualization`,
//! or `visualization_wasm` features is enabled, storage switches from
//! single-threaded `RefCell<HashMap<...>>` double-buffering to a sharded,
//! thread-safe [`DBDashMap`](crate::utils::dbdashmap::DBDashMap). Same
//! public API, same `Field` semantics.
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

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))] {
        // -----------------------------------------------------------------
        // Parallel / visualization branch — DBDashMap-backed, thread-safe.
        //
        // Direct port of the `SparseGrid2D` parallel branch: two
        // DBDashMaps (`obj2loc` + `loc2objs`) handle their own
        // double-buffering. `lazy_update` / `update` swap their internal
        // shards.
        // -----------------------------------------------------------------

        /// Sparse A5-cell-keyed object field, parallel-safe variant.
        pub struct SparseA5Grid<O: Eq + Hash + Clone + Copy> {
            /// `object → location`. Read with `get_read`; write with `insert` / `remove`.
            pub obj2loc: DBDashMap<O, GeoCell>,
            /// `location → bag of objects`.
            pub loc2objs: DBDashMap<GeoCell, Vec<O>>,
            pub root: GeoCell,
            pub resolution: i32,
        }

        impl<O: Eq + Hash + Clone + Copy> SparseA5Grid<O> {
            /// Whole-planet grid at the given A5 resolution.
            pub fn new(resolution: i32) -> Self {
                Self::new_with_root(WORLD_CELL.into(), resolution)
            }

            /// Grid bounded to the descendants of `root`.
            pub fn new_with_root(root: GeoCell, resolution: i32) -> Self {
                SparseA5Grid {
                    obj2loc: DBDashMap::new(),
                    loc2objs: DBDashMap::new(),
                    root,
                    resolution,
                }
            }

            pub fn apply_to_all_values<F>(&self, closure: F, _option: GridOption)
            where
                F: Fn(&GeoCell, &O) -> Option<O>,
            {
                // The 2D parallel branch ignores GridOption and runs
                // `apply_to_all_keys` on `obj2loc`; we mirror that for
                // surface-compatibility. The closure receives
                // (location, object) — same as the single-threaded
                // branch — because `obj2loc`'s K is the object and V
                // is the location.
                self.obj2loc.apply_to_all_keys(closure);
            }

            pub fn set_object_location(&self, object: O, loc: &GeoCell) {
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

            pub fn remove_object_location(&self, object: O, loc: &GeoCell) {
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

            pub fn get_location(&self, object: &O) -> Option<GeoCell> {
                self.obj2loc.get_read(object).copied()
            }

            pub fn get_location_unbuffered(&self, object: &O) -> Option<GeoCell> {
                self.obj2loc.get_write(object).map(|loc_ref| *loc_ref)
            }

            pub fn get_objects(&self, loc: &GeoCell) -> Option<Vec<O>> {
                match self.loc2objs.get_read(loc) {
                    Some(vec) if !vec.is_empty() => Some(vec.clone()),
                    _ => None,
                }
            }

            pub fn get_objects_unbuffered(&self, loc: &GeoCell) -> Option<Vec<O>> {
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

            pub fn num_objects_at_location(&self, loc: &GeoCell) -> usize {
                self.loc2objs
                    .get_read(loc)
                    .map(|bag| bag.len())
                    .unwrap_or(0)
            }

            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(&GeoCell, &O),
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
                F: Fn(&GeoCell, &O),
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

            pub fn get_empty_bags(&self) -> Vec<GeoCell> {
                let mut empty = Vec::new();
                if let Some(cells) = self.all_cells() {
                    for cell in cells {
                        match self.loc2objs.get_read(&cell) {
                            Some(bag) if !bag.is_empty() => {}
                            _ => empty.push(cell),
                        }
                    }
                }
                empty
            }

            pub fn get_random_empty_bag(&self) -> Option<GeoCell> {
                let empty = self.get_empty_bags();
                if empty.is_empty() {
                    return None;
                }
                let mut rng = rand::rng();
                Some(empty[rng.random_range(0..empty.len())])
            }

            fn collect_objects(&self, cells: &[GeoCell]) -> Vec<O> {
                let mut out = Vec::new();
                for cell in cells {
                    if let Some(bag) = self.loc2objs.get_read(cell) {
                        out.extend(bag.iter().copied());
                    }
                }
                out
            }
        }

        impl<O: Eq + Hash + Clone + Copy> Field for SparseA5Grid<O> {
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

        /// Sparse A5-cell-keyed object field, single-threaded variant.
        pub struct SparseA5Grid<O: Eq + Hash + Clone + Copy> {
            /// Two buffers: index 0 = read, index 1 = write. Swapped on
            /// `lazy_update` / `update`.
            pub locs: Vec<RefCell<HashMap<GeoCell, Vec<O>>>>,
            read: usize,
            write: usize,
            pub root: GeoCell,
            pub resolution: i32,
        }

        impl<O: Eq + Hash + Clone + Copy> SparseA5Grid<O> {
            pub fn new(resolution: i32) -> Self {
                Self::new_with_root(WORLD_CELL.into(), resolution)
            }

            pub fn new_with_root(root: GeoCell, resolution: i32) -> Self {
                SparseA5Grid {
                    locs: vec![
                        RefCell::new(HashMap::new()),
                        RefCell::new(HashMap::new()),
                    ],
                    read: 0,
                    write: 1,
                    root,
                    resolution,
                }
            }

            pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
            where
                F: Fn(&GeoCell, &O) -> Option<O>,
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

            pub fn get_location(&self, object: &O) -> Option<GeoCell> {
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

            pub fn get_location_unbuffered(&self, object: &O) -> Option<GeoCell> {
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

            pub fn num_objects_at_location(&self, loc: &GeoCell) -> usize {
                self.locs[self.read]
                    .borrow()
                    .get(loc)
                    .map(|bag| bag.len())
                    .unwrap_or(0)
            }

            pub fn get_objects(&self, loc: &GeoCell) -> Option<Vec<O>> {
                self.locs[self.read].borrow().get(loc).cloned()
            }

            pub fn get_objects_unbuffered(&self, loc: &GeoCell) -> Option<Vec<O>> {
                self.locs[self.write].borrow().get(loc).cloned()
            }

            pub fn get_empty_bags(&self) -> Vec<GeoCell> {
                let mut empty = Vec::new();
                let rlocs = self.locs[self.read].borrow();
                if let Some(cells) = self.all_cells() {
                    for cell in cells {
                        match rlocs.get(&cell) {
                            Some(bag) if !bag.is_empty() => {}
                            _ => empty.push(cell),
                        }
                    }
                }
                empty
            }

            pub fn get_random_empty_bag(&self) -> Option<GeoCell> {
                let empty = self.get_empty_bags();
                if empty.is_empty() {
                    return None;
                }
                let mut rng = rand::rng();
                Some(empty[rng.random_range(0..empty.len())])
            }

            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(&GeoCell, &O),
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
                F: Fn(&GeoCell, &O),
            {
                let locs = self.locs[self.write].borrow();
                for (key, bag) in locs.iter() {
                    for obj in bag {
                        closure(key, obj);
                    }
                }
            }

            pub fn set_object_location(&self, object: O, loc: &GeoCell) {
                let mut locs = self.locs[self.write].borrow_mut();
                match locs.get_mut(loc) {
                    Some(bag) => bag.push(object),
                    None => {
                        locs.insert(*loc, vec![object]);
                    }
                }
            }

            pub fn remove_object_location(&self, object: O, loc: &GeoCell) {
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
                let mut empty_keys: Vec<GeoCell> = Vec::new();
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

            fn collect_objects(&self, cells: &[GeoCell]) -> Vec<O> {
                let rlocs = self.locs[self.read].borrow();
                let mut out = Vec::new();
                for cell in cells {
                    if let Some(bag) = rlocs.get(cell) {
                        out.extend(bag.iter().copied());
                    }
                }
                out
            }
        }

        impl<O: Eq + Hash + Clone + Copy> Field for SparseA5Grid<O> {
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
// Storage-agnostic shared API (read-only; depends only on root + resolution).
// Defined once here; both backends pick this up.
// ---------------------------------------------------------------------------

impl<O: Eq + Hash + Clone + Copy> SparseA5Grid<O> {
    /// All cells covered by this grid (descendants of `root` at `resolution`).
    pub fn all_cells(&self) -> Option<Vec<GeoCell>> {
        self.root.children(self.resolution)
    }

    /// Bounding-box predicate: is `loc` a cell of this grid?
    ///
    /// Returns `true` when `loc.resolution() == self.resolution` **and**
    /// `loc` is a descendant of `self.root`. With `root = WORLD_CELL` the
    /// only check is the resolution match.
    ///
    /// Following the krABMaga convention, `set_object_location` does not
    /// validate locations on insert — agents are expected to call
    /// `contains` before stepping (see `Field2D`'s `toroidal` flag and the
    /// manual bounds checks in the canonical `wolfsheepgrass` example).
    pub fn contains(&self, loc: &GeoCell) -> bool {
        if loc.resolution() != self.resolution {
            return false;
        }
        if self.root.is_world_cell() {
            return true;
        }
        let root_res = self.root.resolution();
        if root_res > self.resolution {
            return false;
        }
        loc.parent(root_res) == Some(self.root)
    }

    // ---------- Cell-only spatial helpers (raw `GeoCell`s) ----------

    pub fn cell_neighbors(&self, loc: &GeoCell) -> Option<Vec<GeoCell>> {
        query::neighbors(loc)
    }

    pub fn cell_vertex_neighbors(&self, loc: &GeoCell) -> Option<Vec<GeoCell>> {
        query::vertex_neighbors(loc)
    }

    pub fn cell_grid_disk(&self, loc: &GeoCell, k: usize) -> Option<Vec<GeoCell>> {
        query::grid_disk(loc, k)
    }

    pub fn cell_spherical_cap(&self, loc: &GeoCell, radius_meters: f64) -> Option<Vec<GeoCell>> {
        query::spherical_cap(loc, radius_meters)
    }

    pub fn lonlat_to_cell(&self, longitude: f64, latitude: f64) -> Option<GeoCell> {
        GeoCell::from_lon_lat(longitude, latitude, self.resolution)
    }

    // ---------- Object-returning spatial queries (read buffer) ----------

    pub fn get_neighbors(&self, loc: &GeoCell) -> Vec<O> {
        match self.cell_neighbors(loc) {
            Some(cells) => self.collect_objects(&cells),
            None => Vec::new(),
        }
    }

    pub fn get_vertex_neighbors(&self, loc: &GeoCell) -> Vec<O> {
        match self.cell_vertex_neighbors(loc) {
            Some(cells) => self.collect_objects(&cells),
            None => Vec::new(),
        }
    }

    pub fn get_objects_within_disk(&self, loc: &GeoCell, k: usize) -> Vec<O> {
        match self.cell_grid_disk(loc, k) {
            Some(cells) => self.collect_objects(&cells),
            None => Vec::new(),
        }
    }

    /// Objects within `dist_meters` arc length of `loc`. Mirrors the krABMaga
    /// `Field2D::get_neighbors_within_distance` idiom; on the sphere
    /// "distance" is metric arc length (`bevy_a5::query::spherical_cap`).
    pub fn get_neighbors_within_distance(&self, loc: &GeoCell, dist_meters: f64) -> Vec<O> {
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Exercises the basic insert/lookup/lazy_update lifecycle on whichever
    /// storage backend is active. Always compiles.
    #[test]
    fn single_threaded_round_trip() {
        let grid: SparseA5Grid<u32> = SparseA5Grid::new(2);
        let cells = grid.all_cells().expect("WORLD_CELL has children at res 2");
        assert!(!cells.is_empty());

        for (i, cell) in cells.iter().take(10).enumerate() {
            grid.set_object_location(i as u32, cell);
        }

        // Pre-update: write buffer holds them; read buffer is empty.
        assert_eq!(grid.num_objects(), 0);
        for i in 0..10u32 {
            assert!(grid.get_location_unbuffered(&i).is_some(), "id {i} not in write buffer");
        }

        let mut grid = grid;
        grid.lazy_update();

        // Post-update: everything visible from the read side.
        assert_eq!(grid.num_objects(), 10);
        for (i, cell) in cells.iter().take(10).enumerate() {
            assert_eq!(grid.get(&(i as u32)), Some(i as u32));
            assert_eq!(grid.get_location(&(i as u32)).as_ref(), Some(cell));
        }
    }

    /// Bounding-box predicate against a non-WORLD_CELL root.
    #[test]
    fn contains_subtree_check() {
        let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
        let grid: SparseA5Grid<u32> = SparseA5Grid::new_with_root(london, 4);
        let cells = grid.all_cells().expect("london has res-4 descendants");
        assert!(!cells.is_empty());

        for cell in &cells {
            assert!(grid.contains(cell), "child of london should be in-bounds");
        }
        // Same cell at the wrong resolution: out of bounds.
        let wrong_res = GeoCell::from_lon_lat(-0.1, 51.5, 5).expect("res 5 cell exists");
        assert!(!grid.contains(&wrong_res));
        // A cell on the opposite hemisphere shouldn't be a descendant.
        let antipode = GeoCell::from_lon_lat(180.0, -51.5, 4).expect("antipode resolves");
        assert!(!grid.contains(&antipode));
    }

    /// Concurrent-insert sanity check for the DBDashMap-backed branch.
    ///
    /// Compiles only when storage is parallel-safe — the RefCell branch
    /// isn't `Sync`, so `Arc<SparseA5Grid<_>>: Send` would fail there.
    ///
    /// **Known limitation.** `set_object_location`'s "no entry at `loc`
    /// yet" path is racy: two threads can both observe `None` and both
    /// call `loc2objs.insert(*loc, vec![..])`, where DBDashMap's `insert`
    /// removes-then-inserts and the second writer's vec replaces the
    /// first's, dropping that object. The 2D `SparseGrid2D` parallel
    /// branch has the same race verbatim (`sparse_object_grid_2d.rs`,
    /// line ~185). The test below warms every cell up single-threaded
    /// first, so subsequent concurrent inserts always take the
    /// `Some(vec)` path, which **is** properly serialised by the shard
    /// mutex.
    #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]
    #[test]
    fn parallel_concurrent_inserts_dont_lose_objects() {
        use std::sync::Arc;
        use std::thread;

        const N_THREADS: u32 = 8;
        const PER_THREAD: u32 = 100;

        let grid: SparseA5Grid<u32> = SparseA5Grid::new(2);
        let cells = grid.all_cells().expect("res-2 children");
        assert!(!cells.is_empty());

        // Warm-up: insert a sentinel at every cell so every cell has an
        // entry. This sidesteps the no-entry-yet race documented above.
        // The sentinel ID is well outside the worker-id range.
        const SENTINEL: u32 = u32::MAX;
        for cell in &cells {
            grid.set_object_location(SENTINEL, cell);
        }

        let grid = Arc::new(grid);
        let cells = Arc::new(cells);

        let handles: Vec<_> = (0..N_THREADS)
            .map(|tid| {
                let grid = Arc::clone(&grid);
                let cells = Arc::clone(&cells);
                thread::spawn(move || {
                    for i in 0..PER_THREAD {
                        let id = tid * PER_THREAD + i;
                        // Round-robin across cells so different threads
                        // contend on overlapping shards.
                        let cell = cells[(id as usize) % cells.len()];
                        grid.set_object_location(id, &cell);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().expect("worker panic");
        }

        // Pre-update: every id is reachable from the write buffer.
        for id in 0..N_THREADS * PER_THREAD {
            assert!(
                grid.get_location_unbuffered(&id).is_some(),
                "id {id} missing from write buffer pre-update"
            );
        }

        let mut grid = Arc::try_unwrap(grid)
            .unwrap_or_else(|_| panic!("workers leaked an Arc handle"));
        grid.lazy_update();

        // Post-update: total count == sentinels + concurrent inserts, and
        // every concurrent-insert id is reachable.
        let inserted = (N_THREADS * PER_THREAD) as usize;
        let expected_total = cells.len() + inserted;
        assert_eq!(
            grid.num_objects(),
            expected_total,
            "lost or duplicated entries during concurrent inserts"
        );
        for id in 0..N_THREADS * PER_THREAD {
            assert_eq!(grid.get(&id), Some(id), "id {id} missing post-update");
            let expected_cell = cells[(id as usize) % cells.len()];
            assert_eq!(
                grid.get_location(&id),
                Some(expected_cell),
                "id {id} ended up in the wrong cell"
            );
        }
    }
}

