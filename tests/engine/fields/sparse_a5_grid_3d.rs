#[cfg(test)]
#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
use {
    krabmaga::bevy_a5::prelude::GeoCell,
    krabmaga::engine::fields::field::Field,
    krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::fields::sparse_a5_grid_3d::{A5Cell3D, SparseA5Grid3D},
    std::cell::Cell,
};

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
fn boundaries() -> Vec<f64> {
    vec![0.0, 1_000.0, 10_000.0, 50_000.0]
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
fn london_3d_grid() -> (SparseA5Grid3D<u32>, Vec<A5Cell3D>) {
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new_with_root(london, 4, Some(boundaries()));
    let locs = grid.all_locations().expect("london has res-4 descendants");
    assert!(!locs.is_empty());
    (grid, locs)
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_layers_and_altitudes() {
    let grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new(2, Some(boundaries()));

    assert_eq!(grid.num_layers(), 3);
    assert_eq!(grid.floor(), Some(0.0));
    assert_eq!(grid.ceiling(), Some(50_000.0));

    // In-range altitudes resolve to consistent layers.
    assert_eq!(grid.layer_for_altitude(0.0), Some(0));
    assert_eq!(grid.layer_for_altitude(500.0), Some(0));
    assert_eq!(grid.layer_for_altitude(1_000.0), Some(1));
    assert_eq!(grid.layer_for_altitude(9_999.99), Some(1));
    assert_eq!(grid.layer_for_altitude(10_000.0), Some(2));
    // Ceiling is inclusive in the top layer.
    assert_eq!(grid.layer_for_altitude(50_000.0), Some(2));

    // Out-of-domain altitudes return None.
    assert_eq!(grid.layer_for_altitude(-1.0), None);
    assert_eq!(grid.layer_for_altitude(50_001.0), None);
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_unbounded_altitude_is_single_layer() {
    let grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new(2, None);
    assert_eq!(grid.num_layers(), 1);
    assert_eq!(grid.floor(), None);
    assert_eq!(grid.ceiling(), None);
    // Any altitude maps to the single layer.
    assert_eq!(grid.layer_for_altitude(-1e9), Some(0));
    assert_eq!(grid.layer_for_altitude(0.0), Some(0));
    assert_eq!(grid.layer_for_altitude(1e9), Some(0));

    // A single-entry vec normalises to None (single layer).
    let single: SparseA5Grid3D<u32> = SparseA5Grid3D::new(2, Some(vec![100.0]));
    assert_eq!(single.num_layers(), 1);
    assert_eq!(single.floor(), None);
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
#[should_panic(expected = "layer boundaries must be strictly ascending")]
fn sparse_a5_grid_3d_rejects_non_ascending_boundaries() {
    let _grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new(2, Some(vec![0.0, 100.0, 50.0]));
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_cell_3d_at_combines_cell_and_layer() {
    let grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new(3, Some(boundaries()));
    let loc = grid
        .cell_3d_at(-0.1, 51.5, 5_000.0)
        .expect("london at 5km resolves");
    assert_eq!(loc.cell.resolution(), 3);
    assert_eq!(loc.layer, 1);
    assert!(grid.contains(&loc));

    // Out-of-domain altitude → None.
    assert!(grid.cell_3d_at(-0.1, 51.5, 1_000_000.0).is_none());
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_top_layer_lifecycle() {
    // The ceiling altitude is inclusive in the top layer — agents that hit
    // exactly the ceiling must round-trip through the grid like any other.
    let grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new(3, Some(boundaries()));
    let ceiling = grid.ceiling().expect("bounded grid has a ceiling");
    let top = grid
        .cell_3d_at(-0.1, 51.5, ceiling)
        .expect("ceiling altitude resolves");
    assert_eq!(top.layer, grid.num_layers() - 1);
    assert!(grid.contains(&top));

    grid.set_object_location(99, &top);
    let mut grid = grid;
    grid.update();

    assert_eq!(grid.get_location(&99).as_ref(), Some(&top));
    assert_eq!(
        grid.num_objects_at_location(&top),
        1,
        "object inserted at the ceiling layer must be retrievable from it"
    );
    let bag = grid.get_objects(&top).expect("top-layer bag populated");
    assert!(bag.contains(&99));
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_bags_lifecycle() {
    let (grid, locs) = london_3d_grid();
    let total = locs.len();

    assert_eq!(grid.get_empty_bags().len(), total);
    let pick = grid.get_random_empty_bag().expect("non-empty");
    assert!(grid.contains(&pick));

    grid.set_object_location(7, &pick);
    grid.set_object_location(8, &pick);

    // Pre-update: write buffer holds them, read is empty.
    assert!(grid.get_location_unbuffered(&7).is_some());
    assert!(grid.get_unbuffered(&7).is_some());
    assert!(grid.get_location(&7).is_none());
    assert!(grid.get(&7).is_none());
    assert_eq!(grid.num_objects(), 0);

    let mut grid = grid;
    grid.update();

    // After update(): read buffer has both objects; num_objects counts the
    // read buffer.
    assert_eq!(grid.num_objects(), 2);
    assert_eq!(grid.num_objects_at_location(&pick), 2);
    assert_eq!(grid.get(&7), Some(7));
    assert_eq!(grid.get_location(&7).as_ref(), Some(&pick));
    let bag = grid.get_objects(&pick).expect("bag populated");
    assert!(bag.contains(&7) && bag.contains(&8));
    assert_eq!(grid.get_empty_bags().len(), total - 1);

    // remove_object_location targets the *write* buffer (mirrors the
    // single-threaded `SparseGrid2D` semantics). Stage again, then remove,
    // and verify the write-side view.
    grid.set_object_location(7, &pick);
    grid.set_object_location(8, &pick);
    grid.remove_object_location(7, &pick);
    let write_bag = grid
        .get_objects_unbuffered(&pick)
        .expect("write buffer still has obj 8");
    assert!(!write_bag.contains(&7));
    assert!(write_bag.contains(&8));

    // remove_object walks every cell in the write buffer and drops empties.
    grid.remove_object(&8);
    assert!(
        grid.get_objects_unbuffered(&pick).is_none(),
        "write bag should be removed after remove_object empties it"
    );

    // After lazy_update, the now-empty write buffer is swapped into read.
    grid.lazy_update();
    assert!(grid.get_objects(&pick).is_none());
    assert_eq!(grid.num_objects(), 0);
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Tagged {
    id: u32,
    flag: bool,
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_apply_and_iter() {
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid3D<Tagged> = SparseA5Grid3D::new_with_root(london, 4, Some(boundaries()));
    let locs = grid.all_locations().expect("res-4 locs");
    let n = locs.len().min(6);

    for (i, loc) in locs.iter().take(n).enumerate() {
        grid.set_object_location(
            Tagged {
                id: i as u32,
                flag: false,
            },
            loc,
        );
    }

    // iter↔get round-trip on the write buffer.
    let unbuf_count = Cell::new(0usize);
    grid.iter_objects_unbuffered(|loc, t| {
        let bag = grid
            .get_objects_unbuffered(loc)
            .expect("iter handed us a loc that get can't find");
        assert!(bag.iter().any(|x| x.id == t.id));
        unbuf_count.set(unbuf_count.get() + 1);
    });
    assert_eq!(unbuf_count.get(), n);

    grid.apply_to_all_values(
        |_loc, t| {
            Some(Tagged {
                id: t.id,
                flag: true,
            })
        },
        GridOption::WRITE,
    );
    grid.iter_objects_unbuffered(|_loc, t| assert!(t.flag));

    let mut grid = grid;
    grid.lazy_update();

    // iter↔get round-trip on the read buffer.
    let read_count = Cell::new(0usize);
    grid.iter_objects(|loc, t| {
        assert!(t.flag);
        let bag = grid
            .get_objects(loc)
            .expect("iter handed us a loc that get can't find");
        assert!(bag.iter().any(|x| x.id == t.id));
        read_count.set(read_count.get() + 1);
    });
    assert_eq!(read_count.get(), n);

    grid.apply_to_all_values(
        |_loc, t| {
            Some(Tagged {
                id: t.id,
                flag: false,
            })
        },
        GridOption::READ,
    );
    grid.iter_objects(|_loc, t| assert!(!t.flag));

    // Re-stage and exercise READWRITE.
    for (i, loc) in locs.iter().take(n).enumerate() {
        grid.set_object_location(
            Tagged {
                id: i as u32,
                flag: false,
            },
            loc,
        );
    }
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
    grid.iter_objects(|_loc, t| assert!(t.flag));
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_neighbors_stay_in_layer() {
    let (grid, locs) = london_3d_grid();
    // Pick a centre with layer > 0 so a bug where lift_to_layer hard-coded
    // 0 would fail this test.
    let centre = *locs
        .iter()
        .find(|l| l.layer > 0)
        .expect("3D grid has multiple layers");
    assert!(centre.layer > 0);

    let neigh = grid.cell_neighbors(&centre).expect("centre has neighbours");
    assert!(!neigh.is_empty());
    for n in &neigh {
        assert_eq!(
            n.layer, centre.layer,
            "neighbour lifted to wrong layer (expected {}, got {})",
            centre.layer, n.layer
        );
    }

    // Vertex neighbours and disk cells must also stay in the centre's layer.
    if let Some(vneigh) = grid.cell_vertex_neighbors(&centre) {
        for v in &vneigh {
            assert_eq!(v.layer, centre.layer);
        }
    }
    let disk_cells = grid.cell_grid_disk(&centre, 1).expect("disk has cells");
    for d in &disk_cells {
        assert_eq!(d.layer, centre.layer);
    }

    for (i, loc) in neigh.iter().enumerate() {
        grid.set_object_location(100 + i as u32, loc);
    }
    grid.set_object_location(0, &centre);

    let mut grid = grid;
    grid.update();

    let n_objs = grid.get_neighbors(&centre);
    assert_eq!(n_objs.len(), neigh.len());

    let disk = grid.get_objects_within_disk(&centre, 1);
    assert!(disk.contains(&0));
    assert_eq!(disk.len(), neigh.len() + 1);

    let none = grid.get_neighbors_within_distance(&centre, 0.0);
    assert!(none.is_empty());

    let near = grid.get_neighbors_within_distance(&centre, 1_000_000.0);
    assert!(near.contains(&0));
}

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
#[test]
fn sparse_a5_grid_3d_contains_layer_and_subtree() {
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid3D<u32> = SparseA5Grid3D::new_with_root(london, 4, Some(boundaries()));

    let cell = GeoCell::from_lon_lat(-0.1, 51.5, 4).expect("res 4 cell");
    assert!(grid.contains(&A5Cell3D::new(cell, 0)));
    assert!(grid.contains(&A5Cell3D::new(cell, 2)));
    // Out-of-range layer.
    assert!(!grid.contains(&A5Cell3D::new(cell, 3)));
    // Wrong cell resolution.
    let wrong_res = GeoCell::from_lon_lat(-0.1, 51.5, 5).expect("res 5 cell");
    assert!(!grid.contains(&A5Cell3D::new(wrong_res, 0)));
    // Outside the london subtree.
    let antipode = GeoCell::from_lon_lat(180.0, -51.5, 4).expect("antipode");
    assert!(!grid.contains(&A5Cell3D::new(antipode, 0)));

    // World-cell root short-circuits the subtree check, but still rejects
    // wrong resolutions and out-of-range layers.
    let world: SparseA5Grid3D<u32> = SparseA5Grid3D::new(4, Some(boundaries()));
    assert!(world.contains(&A5Cell3D::new(cell, 1)));
    assert!(!world.contains(&A5Cell3D::new(wrong_res, 1)));

    // Root deeper than resolution must be rejected as out-of-bounds.
    let deep_root = GeoCell::from_lon_lat(-0.1, 51.5, 4).expect("deep root");
    let bad_grid: SparseA5Grid3D<u32> =
        SparseA5Grid3D::new_with_root(deep_root, 2, Some(boundaries()));
    let res2 = GeoCell::from_lon_lat(-0.1, 51.5, 2).expect("res 2 cell");
    assert!(!bad_grid.contains(&A5Cell3D::new(res2, 0)));
}
