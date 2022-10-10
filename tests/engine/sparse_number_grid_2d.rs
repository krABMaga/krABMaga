#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
static HEIGHT: i32 = 10;
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
static WIDTH: i32 = 10;

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
use {
    krabmaga::engine::{
        fields::field::Field, fields::grid_option::GridOption,
        fields::sparse_number_grid_2d::SparseNumberGrid2D, location::Int2D,
    },
    krabmaga::*,
};

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn sparse_number_grid_2d() {
    let mut grid: SparseNumberGrid2D<u16> = SparseNumberGrid2D::new(WIDTH, HEIGHT);

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location(0, &loc);
        }
    }

    let bags = grid.get_empty_bags();
    assert_eq!(bags.len(), 100);

    grid.update();
    let bags = grid.get_empty_bags();
    assert_eq!(bags.len(), 0);

    grid.apply_to_all_values(|_value| 1, GridOption::WRITE);

    grid.iter_values_unbuffered(|_, val| {
        assert_eq!(*val, 1);
    });

    grid.lazy_update();

    grid.iter_values(|_, val| {
        assert_eq!(*val, 1);
    });

    grid.apply_to_all_values(
        |value| {
            let val = *value;
            assert_eq!(val, 1);
            val + 1
        },
        GridOption::READWRITE,
    );

    grid.update();

    grid.apply_to_all_values(
        |value| {
            assert_eq!(*value, 2);
            *value + 1
        },
        GridOption::READ,
    );

    grid.iter_values_unbuffered(|loc, val| {
        let value = grid.get_value_unbuffered(&loc).unwrap();
        assert_eq!(*val, value);
        assert_eq!(value, 2);
    });

    grid.iter_values(|loc, val| {
        let value = grid.get_value(&loc).unwrap();
        assert_eq!(*val, value);
        assert_eq!(value, 3);
    });

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location((i * j) as u16, &loc);
        }
    }

    grid.lazy_update();

    grid.iter_values(|loc, val| {
        let value = grid.get_value(&loc).unwrap();
        assert_eq!(*val, value);
        assert_eq!((loc.x * loc.y) as u16, value);
    });
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn sparse_number_grid_2d_bags() {
    let mut grid: SparseNumberGrid2D<u16> = SparseNumberGrid2D::new(WIDTH, HEIGHT);

    let all = grid.get_empty_bags();
    assert_eq!((HEIGHT * WIDTH) as usize, all.len());

    let loc = grid.get_random_empty_bag();
    assert!(None != loc);
    let loc = loc.unwrap();
    grid.set_value_location(10, &loc);

    let value = grid.get_value_unbuffered(&loc);
    assert!(None != value);
    assert_eq!(Some(10), value);
    grid.remove_value_location(&loc);
    let value = grid.get_value_unbuffered(&loc);
    assert!(None == value);

    grid.set_value_location(10, &loc);
    grid.update();
    let all = grid.get_empty_bags();
    assert_eq!((HEIGHT * WIDTH - 1) as usize, all.len());

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location(0, &loc);
        }
    }

    let loc = grid.get_location_unbuffered(0);
    assert!(loc.is_some());

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(1..WIDTH);
    let j = rng.gen_range(1..HEIGHT);
    let loc = Int2D { x: i, y: j };

    grid.set_value_location(5, &loc);
    let loc2 = grid.get_location_unbuffered(5);
    assert!(None != loc2);
    let loc2 = loc2.unwrap();
    assert_eq!(loc2.x, i);
    assert_eq!(loc2.y, j);

    assert!(grid.get_location_unbuffered(6).is_none());

    grid.lazy_update();

    let loc = grid.get_location(0);
    assert!(loc.is_some());

    let loc2 = grid.get_location(5);
    assert!(None != loc2);
    let loc2 = loc2.unwrap();
    assert_eq!(loc2.x, i);
    assert_eq!(loc2.y, j);

    assert!(grid.get_location(6).is_none());

    let none = grid.get_empty_bags();
    assert_eq!(0, none.len());
}
