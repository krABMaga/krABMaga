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
    rust_ab::engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
    rust_ab::engine::fields::field::Field, rust_ab::engine::fields::grid_option::GridOption,
    rust_ab::engine::location::Int2D,
};

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn dense_number_grid_2d_apply() {
    let mut grid: DenseNumberGrid2D<u16> = DenseNumberGrid2D::new(WIDTH, HEIGHT);

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location(0, &loc);
        }
    }

    grid.lazy_update();

    grid.apply_to_all_values(|_value| 1, GridOption::WRITE);

    grid.lazy_update();

    grid.apply_to_all_values(
        |value| {
            let val = *value;
            assert_eq!(val, 1);
            val + 1
        },
        GridOption::READWRITE,
    );

    grid.lazy_update();

    grid.apply_to_all_values(|value| *value + 1, GridOption::READ);

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let loc = Int2D { x: i, y: j };
            let val = grid.get_value(&loc).unwrap();
            assert_eq!(val, 3);
        }
    }

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location((i * j) as u16, &loc);
        }
    }

    grid.iter_values_unbuffered(|loc, val| {
        let value = grid.get_value_unbuffered(&loc).unwrap();
        assert_eq!(*val, value[0]);
        assert_eq!((loc.x * loc.y) as u16, value[0]);
    });

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
fn dense_number_grid_2d_bags() {
    let mut grid: DenseNumberGrid2D<u16> = DenseNumberGrid2D::new(WIDTH, HEIGHT);

    let all = grid.get_empty_bags();
    assert_eq!((HEIGHT * WIDTH) as usize, all.len());

    let loc = grid.get_random_empty_bag();
    assert!(None != loc);
    grid.set_value_location(10, &(loc.unwrap()));
    grid.update();
    let all = grid.get_empty_bags();
    assert_eq!((HEIGHT * WIDTH - 1) as usize, all.len());

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location(0, &loc);
        }
    }

    grid.update();
    let none = grid.get_empty_bags();
    assert_eq!(0, none.len());
}
