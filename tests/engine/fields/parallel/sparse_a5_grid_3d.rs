#[cfg(test)]
#[cfg(all(feature = "gis", feature = "parallel"))]
use {
    krabmaga::bevy_a5::prelude::GeoCell,
    krabmaga::engine::fields::field::Field,
    krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::fields::sparse_a5_grid_3d::{A5Cell3D, SparseA5Grid3D},
    std::sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(all(feature = "gis", feature = "parallel"))]
fn boundaries() -> Vec<f64> {
    vec![0.0, 1_000.0, 10_000.0, 50_000.0]
}

#[cfg(all(feature = "gis", feature = "parallel"))]
fn london_3d_grid() -> (SparseA5Grid3D<u32>, Vec<A5Cell3D>) {
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new_with_root(london, 4, Some(boundaries()));
    let locs = grid.all_locations().expect("london has res-4 descendants");
    (grid, locs)
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_3d_bags_parallel() {
    let (grid, locs) = london_3d_grid();
    let total = locs.len();
    assert_eq!(grid.get_empty_bags().len(), total);

    let pick = grid.get_random_empty_bag().expect("non-empty");
    grid.set_object_location(7, &pick);
    grid.set_object_location(8, &pick);
    let mut grid = grid;
    grid.update();

    assert_eq!(grid.num_objects(), 2);
    assert_eq!(grid.num_objects_at_location(&pick), 2);
    let bag = grid.get_objects(&pick).expect("bag populated");
    assert!(bag.contains(&7) && bag.contains(&8));
    assert_eq!(grid.get_empty_bags().len(), total - 1);
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Tagged {
    id: u32,
    flag: bool,
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_3d_apply_parallel() {
    // See `sparse_a5_grid_apply_parallel` for the full rationale. Short
    // version: the parallel branch routes apply through
    // `obj2loc.apply_to_all_keys`, ignores GridOption, and never touches
    // loc2objs. After apply+lazy_update, rewritten keys are visible via
    // `get`; `iter_objects` would iterate the now-empty loc2objs.r_shards
    // and pass vacuously.
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid3D<Tagged> = SparseA5Grid3D::new_with_root(london, 4, Some(boundaries()));
    let locs = grid.all_locations().expect("res-4 locs");
    let n = locs.len().min(6) as u32;

    for i in 0..n {
        grid.set_object_location(Tagged { id: i, flag: false }, &locs[i as usize]);
    }
    let mut grid = grid;
    grid.lazy_update();

    // WRITE — rewrite flag false → true.
    grid.apply_to_all_values(
        |_loc, t| {
            Some(Tagged {
                id: t.id,
                flag: true,
            })
        },
        GridOption::WRITE,
    );
    grid.lazy_update();
    for i in 0..n {
        assert_eq!(
            grid.get(&Tagged { id: i, flag: true }),
            Some(Tagged { id: i, flag: true }),
            "apply WRITE should have rewritten id={i} key to flag=true"
        );
        assert!(
            grid.get(&Tagged { id: i, flag: false }).is_none(),
            "old flag=false key for id={i} should be gone from obj2loc"
        );
    }

    // READ — parallel branch treats READ identically.
    grid.apply_to_all_values(
        |_loc, t| {
            Some(Tagged {
                id: t.id,
                flag: false,
            })
        },
        GridOption::READ,
    );
    grid.lazy_update();
    for i in 0..n {
        assert_eq!(
            grid.get(&Tagged { id: i, flag: false }),
            Some(Tagged { id: i, flag: false }),
            "apply READ should have rewritten id={i} back to flag=false"
        );
    }

    // READWRITE — same again.
    grid.apply_to_all_values(
        |_loc, t| {
            Some(Tagged {
                id: t.id,
                flag: true,
            })
        },
        GridOption::READWRITE,
    );
    grid.lazy_update();
    for i in 0..n {
        assert_eq!(
            grid.get(&Tagged { id: i, flag: true }),
            Some(Tagged { id: i, flag: true }),
            "apply READWRITE should have rewritten id={i} to flag=true"
        );
    }
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_3d_gets_parallel() {
    let (grid, locs) = london_3d_grid();
    let loc = locs[0];

    assert!(grid.get_objects(&loc).is_none());
    grid.set_object_location(42, &loc);

    // Write-buffer-only state.
    assert_eq!(
        grid.get_objects_unbuffered(&loc).unwrap().first().copied(),
        Some(42)
    );
    assert!(grid.get_objects(&loc).is_none());
    assert!(grid.get(&42).is_none());
    let unbuf_loc = grid.get_location_unbuffered(&42);
    assert_eq!(unbuf_loc.as_ref(), Some(&loc));
    assert!(grid.get_location(&42).is_none());

    let mut grid = grid;
    grid.update();

    assert_eq!(grid.get(&42), Some(42));
    assert_eq!(grid.get_location(&42).as_ref(), Some(&loc));

    grid.remove_object(&42);
    grid.lazy_update();
    assert!(grid.get(&42).is_none());
    assert!(grid.get_objects(&loc).is_none());
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_3d_iter_and_spatial_parallel() {
    let (grid, locs) = london_3d_grid();
    // Pick a non-zero layer to guard against a hard-coded layer-0 lift.
    let centre = *locs
        .iter()
        .find(|l| l.layer > 0)
        .expect("3D grid has multiple layers");
    assert!(centre.layer > 0);

    let neigh = grid.cell_neighbors(&centre).expect("centre has neighbours");
    for n in &neigh {
        assert_eq!(n.layer, centre.layer);
    }

    for (i, l) in neigh.iter().enumerate() {
        grid.set_object_location(100 + i as u32, l);
    }
    grid.set_object_location(0, &centre);

    // iter_objects_unbuffered hits every shard. Round-tripping into
    // get_objects_unbuffered inside the closure would deadlock on the
    // shard mutex, so just count here.
    let count = AtomicUsize::new(0);
    grid.iter_objects_unbuffered(|_loc, _obj| {
        count.fetch_add(1, Ordering::Relaxed);
    });
    assert_eq!(count.load(Ordering::Relaxed), neigh.len() + 1);

    let mut grid = grid;
    grid.update();

    // iter↔get round-trip on the read side.
    let count = AtomicUsize::new(0);
    grid.iter_objects(|loc, obj| {
        let bag = grid
            .get_objects(loc)
            .expect("iter handed us a loc that get can't find");
        assert!(bag.contains(obj));
        count.fetch_add(1, Ordering::Relaxed);
    });
    assert_eq!(count.load(Ordering::Relaxed), neigh.len() + 1);

    let disk = grid.get_objects_within_disk(&centre, 1);
    assert!(disk.contains(&0));
    assert_eq!(disk.len(), neigh.len() + 1);
    assert!(grid
        .get_neighbors_within_distance(&centre, 1_000_000.0)
        .contains(&0));
    assert!(grid.get_neighbors_within_distance(&centre, 0.0).is_empty());
}
