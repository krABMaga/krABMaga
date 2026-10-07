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
    krabmaga::bevy_a5::prelude::GeoCell, krabmaga::engine::fields::field::Field,
    krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::fields::sparse_a5_grid::SparseA5Grid, std::cell::Cell,
};

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
const RES: i32 = 2;

#[cfg(all(
    feature = "gis",
    not(any(
        feature = "visualization",
        feature = "visualization_wasm",
        feature = "parallel"
    ))
))]
fn london_root_grid() -> (SparseA5Grid<u32>, Vec<GeoCell>) {
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid<u32> = SparseA5Grid::new_with_root(london, 4);
    let cells = grid.all_cells().expect("london has res-4 descendants");
    assert!(!cells.is_empty(), "london should have descendants at res 4");
    (grid, cells)
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
fn sparse_a5_grid_bags() {
    let (grid, cells) = london_root_grid();
    let total = cells.len();

    let empty = grid.get_empty_bags();
    assert_eq!(empty.len(), total);

    let pick = grid.get_random_empty_bag().expect("non-empty universe");
    assert!(grid.contains(&pick));

    grid.set_object_location(0, &pick);

    // Pre-update: write buffer has it, read buffer doesn't.
    assert_eq!(grid.get_location_unbuffered(&0).as_ref(), Some(&pick));
    assert!(grid.get_location(&0).is_none());
    assert!(grid.get_unbuffered(&0).is_some());
    assert!(grid.get(&0).is_none());
    assert_eq!(grid.num_objects(), 0);

    let mut grid = grid;
    grid.update();

    assert_eq!(grid.get_location(&0).as_ref(), Some(&pick));
    assert_eq!(grid.get(&0), Some(0));
    assert_eq!(grid.num_objects(), 1);
    assert_eq!(grid.num_objects_at_location(&pick), 1);
    assert_eq!(grid.get_objects(&pick).map(|v| v.len()), Some(1));
    assert_eq!(grid.get_empty_bags().len(), total - 1);

    // Remove via location: location-keyed and object-keyed lookups both vanish.
    grid.remove_object_location(0, &pick);
    grid.lazy_update();
    assert!(grid.get_objects(&pick).is_none());
    assert_eq!(grid.num_objects(), 0);
    assert_eq!(grid.get_empty_bags().len(), total);
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
fn sparse_a5_grid_remove_object_scans_all_cells() {
    let (grid, cells) = london_root_grid();
    let a = cells[0];
    let b = cells[1 % cells.len()];

    grid.set_object_location(7, &a);
    if a != b {
        grid.set_object_location(8, &b);
    }
    let mut grid = grid;
    grid.update();

    assert_eq!(grid.get_location(&7).as_ref(), Some(&a));

    grid.remove_object(&7);
    grid.lazy_update();

    assert!(grid.get_location(&7).is_none());
    assert!(grid.get(&7).is_none());
    assert_eq!(grid.get_objects(&a), None);
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
fn sparse_a5_grid_apply_and_iter() {
    let london = GeoCell::from_lon_lat(-0.1, 51.5, 1).expect("london resolves");
    let grid: SparseA5Grid<Tagged> = SparseA5Grid::new_with_root(london, 4);
    let cells = grid.all_cells().expect("london has res-4 descendants");
    let n = cells.len().min(8);

    for (i, cell) in cells.iter().take(n).enumerate() {
        grid.set_object_location(
            Tagged {
                id: i as u32,
                flag: false,
            },
            cell,
        );
    }

    // iter_objects_unbuffered visits every staged entry; for each, the loc
    // it hands us must be a valid key into get_objects_unbuffered, and the
    // returned bag must contain the same object.
    let count = Cell::new(0usize);
    grid.iter_objects_unbuffered(|loc, t| {
        let bag = grid
            .get_objects_unbuffered(loc)
            .expect("iter handed us a loc that get can't find");
        assert!(bag.iter().any(|x| x.id == t.id));
        count.set(count.get() + 1);
    });
    assert_eq!(count.get(), n);

    // WRITE: rewrite objects in the write buffer.
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

    // Read-side iter↔get round-trip on the flagged copies.
    let count = Cell::new(0usize);
    grid.iter_objects(|loc, t| {
        assert!(t.flag);
        let bag = grid
            .get_objects(loc)
            .expect("iter handed us a loc that get can't find");
        assert!(bag.iter().any(|x| x.id == t.id));
        count.set(count.get() + 1);
    });
    assert_eq!(count.get(), n);

    // READ: rewrite objects in the read buffer.
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

    // Stage new entries in write, then READWRITE merges new+existing.
    for (i, cell) in cells.iter().take(n).enumerate() {
        grid.set_object_location(
            Tagged {
                id: i as u32,
                flag: false,
            },
            cell,
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
fn sparse_a5_grid_update_copies_write_into_read() {
    let (grid, cells) = london_root_grid();
    let cell = cells[0];

    grid.set_object_location(11, &cell);
    grid.set_object_location(12, &cell);

    let mut grid = grid;
    grid.update();

    // update() (vs lazy_update) copies write → read and clears write.
    let read_bag = grid.get_objects(&cell).expect("read bag populated");
    assert_eq!(read_bag.len(), 2);
    assert!(
        grid.get_objects_unbuffered(&cell).is_none(),
        "update() must clear the write buffer"
    );
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
fn sparse_a5_grid_spatial_queries() {
    let (grid, cells) = london_root_grid();
    let centre = cells[0];

    // Drop one object in every direct neighbour of `centre`, plus one in `centre`.
    let neighbours = grid
        .cell_neighbors(&centre)
        .expect("centre has neighbours within london");
    assert!(!neighbours.is_empty());
    for (i, n) in neighbours.iter().enumerate() {
        grid.set_object_location(100 + i as u32, n);
    }
    grid.set_object_location(0, &centre);

    let mut grid = grid;
    grid.update();

    let neigh_objs = grid.get_neighbors(&centre);
    assert_eq!(neigh_objs.len(), neighbours.len());

    let vneighs = grid.cell_vertex_neighbors(&centre);
    if let Some(vneighs) = vneighs {
        let v_objs = grid.get_vertex_neighbors(&centre);
        assert!(v_objs.len() <= vneighs.len());
    }

    // Disk of radius 1 = centre + immediate neighbours.
    let disk_objs = grid.get_objects_within_disk(&centre, 1);
    assert!(disk_objs.contains(&0));
    assert_eq!(disk_objs.len(), 1 + neighbours.len());

    // dist == 0 → no objects.
    let none = grid.get_neighbors_within_distance(&centre, 0.0);
    assert!(none.is_empty());

    // A generous spherical cap pulls in at least the centre object.
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
fn sparse_a5_grid_lonlat_and_contains() {
    let grid: SparseA5Grid<u32> = SparseA5Grid::new(RES);
    let cell = grid
        .lonlat_to_cell(-0.1, 51.5)
        .expect("london resolves at RES");
    assert_eq!(cell.resolution(), RES);
    assert!(grid.contains(&cell));

    // Wrong-resolution cell isn't contained.
    let wrong = GeoCell::from_lon_lat(-0.1, 51.5, RES + 1).expect("res+1 cell exists");
    assert!(!grid.contains(&wrong));
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
fn sparse_a5_grid_contains_rejects_root_deeper_than_resolution() {
    // root at res 4, grid asks for res 2 → root deeper than resolution
    // (this is an unusual misconfiguration, but `contains` must reject it).
    let deep_root = GeoCell::from_lon_lat(-0.1, 51.5, 4).expect("deep root resolves");
    let grid: SparseA5Grid<u32> = SparseA5Grid::new_with_root(deep_root, 2);
    let test = GeoCell::from_lon_lat(-0.1, 51.5, 2).expect("res 2 cell exists");
    assert!(!grid.contains(&test));
}
