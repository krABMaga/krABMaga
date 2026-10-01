#[cfg(test)]
#[cfg(all(feature = "gis", feature = "parallel"))]
use {
    krabmaga::bevy_a5::prelude::GeoCell,
    krabmaga::engine::fields::field::Field,
    krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::fields::sparse_a5_grid::SparseA5Grid,
    std::sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(all(feature = "gis", feature = "parallel"))]
fn london_root_grid() -> (SparseA5Grid<u32>, Vec<GeoCell>) {
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid<u32> = SparseA5Grid::new_with_root(london, 4);
    let cells = grid.all_cells().expect("london has res-4 descendants");
    (grid, cells)
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_bags_parallel() {
    let (grid, cells) = london_root_grid();
    let total = cells.len();

    assert_eq!(grid.get_empty_bags().len(), total);
    let pick = grid.get_random_empty_bag().expect("non-empty");
    grid.set_object_location(0, &pick);
    let mut grid = grid;
    grid.update();

    assert_eq!(grid.num_objects(), 1);
    assert_eq!(grid.num_objects_at_location(&pick), 1);
    assert_eq!(grid.get_empty_bags().len(), total - 1);
    assert_eq!(grid.get_location(&0).as_ref(), Some(&pick));
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Tagged {
    id: u32,
    flag: bool,
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_apply_parallel() {
    // The parallel branch routes apply_to_all_values through
    // `obj2loc.apply_to_all_keys`, which:
    //   - reads from obj2loc.r_shards (data lands there after the first
    //     lazy_update), invokes the closure, and stages rewritten keys
    //     into obj2loc.shards (the write side);
    //   - ignores the GridOption argument entirely;
    //   - never touches `loc2objs`.
    //
    // After the second lazy_update, the rewritten keys are visible via
    // `get()` (which reads obj2loc.r_shards). We verify via `get` because
    // `iter_objects` walks loc2objs.r_shards — which has been swapped to
    // the empty side by the second lazy_update — so it would iterate zero
    // times and a "for-each / assert flag" pattern would pass vacuously.
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid<Tagged> = SparseA5Grid::new_with_root(london, 4);
    let cells = grid.all_cells().expect("res-4 cells");
    let n = cells.len().min(6) as u32;

    for i in 0..n {
        grid.set_object_location(Tagged { id: i, flag: false }, &cells[i as usize]);
    }
    let mut grid = grid;
    grid.lazy_update();

    // Sanity check: get(old_key) is Some before apply runs.
    for i in 0..n {
        assert_eq!(
            grid.get(&Tagged { id: i, flag: false }),
            Some(Tagged { id: i, flag: false }),
            "staged id={i} should be findable before apply"
        );
    }

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

    // READ — parallel branch treats READ identically to WRITE (same
    // underlying `apply_to_all_keys` call). Rewrite flag back to false.
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
        assert!(grid.get(&Tagged { id: i, flag: true }).is_none());
    }

    // READWRITE — same treatment again.
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
fn sparse_a5_grid_gets_parallel() {
    let (grid, cells) = london_root_grid();
    let loc = cells[0];

    assert!(grid.get_objects(&loc).is_none());

    grid.set_object_location(42, &loc);

    // Write-buffer-only state.
    assert!(grid.get_objects_unbuffered(&loc).is_some());
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

    assert_eq!(grid.get_objects(&loc).unwrap().first().copied(), Some(42));
    assert_eq!(grid.get(&42), Some(42));
    assert_eq!(grid.get_location(&42).as_ref(), Some(&loc));

    // remove_object scrubs both maps.
    grid.remove_object(&42);
    grid.lazy_update();
    assert!(grid.get_objects(&loc).is_none());
    assert!(grid.get(&42).is_none());
    assert!(grid.get_location(&42).is_none());
}

#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_iter_and_spatial_parallel() {
    let (grid, cells) = london_root_grid();
    let centre = cells[0];

    let neigh = grid.cell_neighbors(&centre).expect("centre has neighbours");
    for (i, c) in neigh.iter().enumerate() {
        grid.set_object_location(100 + i as u32, c);
    }
    grid.set_object_location(0, &centre);

    // iter_objects_unbuffered hits every shard. (Round-tripping back into
    // get_objects_unbuffered here would deadlock — iter holds the shard
    // mutex and get_write would try to re-acquire it.)
    let count = AtomicUsize::new(0);
    grid.iter_objects_unbuffered(|_loc, _obj| {
        count.fetch_add(1, Ordering::Relaxed);
    });
    assert_eq!(count.load(Ordering::Relaxed), neigh.len() + 1);

    let mut grid = grid;
    grid.update();

    // After update, iter↔get round-trip on the read side.
    let count = AtomicUsize::new(0);
    grid.iter_objects(|loc, obj| {
        let bag = grid
            .get_objects(loc)
            .expect("iter handed us a loc that get can't find");
        assert!(bag.contains(obj));
        count.fetch_add(1, Ordering::Relaxed);
    });
    assert_eq!(count.load(Ordering::Relaxed), neigh.len() + 1);

    // get_neighbors returns objects from the *neighbour* cells of `centre`,
    // not from `centre` itself — so the count is exactly `neigh.len()`.
    assert_eq!(grid.get_neighbors(&centre).len(), neigh.len());
    let disk = grid.get_objects_within_disk(&centre, 1);
    assert!(disk.contains(&0));
    // Disk radius 1 = centre + immediate neighbours.
    assert_eq!(disk.len(), neigh.len() + 1);
    assert!(grid
        .get_neighbors_within_distance(&centre, 1_000_000.0)
        .contains(&0));
    assert!(grid.get_neighbors_within_distance(&centre, 0.0).is_empty());
}

/// Concurrent-insert sanity check for the DBDashMap-backed branch.
///
/// **Known limitation.** `set_object_location`'s "no entry at `loc` yet"
/// path is racy: two threads can both observe `None` and both call
/// `loc2objs.insert(*loc, vec![..])`, where DBDashMap's `insert`
/// removes-then-inserts and the second writer's vec replaces the first's,
/// dropping that object. The 2D `SparseGrid2D` parallel branch has the same
/// race verbatim. The test below warms every cell up single-threaded first,
/// so subsequent concurrent inserts always take the `Some(vec)` path, which
/// **is** properly serialised by the shard mutex.
#[cfg(all(feature = "gis", feature = "parallel"))]
#[test]
fn sparse_a5_grid_concurrent_inserts_dont_lose_objects() {
    use std::sync::Arc;
    use std::thread;

    const N_THREADS: u32 = 8;
    const PER_THREAD: u32 = 100;

    let grid: SparseA5Grid<u32> = SparseA5Grid::new(2);
    let cells = grid.all_cells().expect("res-2 children");
    assert!(!cells.is_empty());

    // Warm-up: insert a sentinel at every cell so every cell has an entry.
    // This sidesteps the no-entry-yet race documented above. The sentinel
    // ID is well outside the worker-id range.
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
                    // Round-robin across cells so different threads contend
                    // on overlapping shards.
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

    let mut grid = Arc::try_unwrap(grid).unwrap_or_else(|_| panic!("workers leaked an Arc handle"));
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
