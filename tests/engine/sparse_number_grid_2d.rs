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
    krabmaga::engine::fields::field::Field, krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::fields::sparse_number_grid_2d::SparseNumberGrid2D,
    krabmaga::engine::location::Int2D,
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

    grid.apply_to_all_values(|_value| 1, GridOption::WRITE);

    grid.update();

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

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            let val = grid.get_value_unbuffered(&loc).unwrap();
            assert_eq!(val, 2);
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            let val = grid.get_value(&loc).unwrap();
            assert_eq!(val, 3);
        }
    }

    let loc = Int2D { x: 4, y: 5 };
    grid.set_value_location(3, &loc);
    grid.remove_value_location(3, &loc);
    let v = grid.get_value_unbuffered(&loc);
    assert!(v.is_none());
    let v = grid.get_value(&loc);
    assert!(v.is_some());

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location((i * j) as u16, &loc);
        }
    }

    grid.lazy_update();

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            let val = grid.get_value(&loc).unwrap();
            assert_eq!(val as i32, i * j);
        }
    }
}
