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
use krabmaga::{
    engine::fields::dense_number_grid_2d::DenseNumberGrid2D, engine::fields::field::Field,
    engine::fields::grid_option::GridOption, engine::location::Int2D, *,
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
        assert_eq!(*val, value);
        assert_eq!((loc.x * loc.y) as u16, value);
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
    assert_eq!(all.len(), (WIDTH * HEIGHT) as usize);

    let random_bag = grid.get_random_empty_bag();
    assert!(random_bag.is_some());
    let bag_loc = random_bag.unwrap();
    grid.set_value_location(10, &bag_loc);

    grid.lazy_update();

    let pos = grid.get_location(10);
    assert_eq!(pos, Some(bag_loc));

    let none = grid.get_location(20);
    assert_eq!(none, None);

    let pos_unbuffered = grid.get_location_unbuffered(10);
    assert_eq!(pos_unbuffered, None);

    grid.set_value_location(20, &Int2D { x: 0, y: 0 });
    let pos_unbuffered_2 = grid.get_location_unbuffered(20);
    assert_eq!(pos_unbuffered_2, Some(Int2D { x: 0, y: 0 }));

    grid.remove_value_location(&Int2D { x: 0, y: 0 });
    let pos_unbuffered_3 = grid.get_location_unbuffered(20);
    assert_eq!(pos_unbuffered_3, None);

    grid.lazy_update();
    let val = grid.get_value(&Int2D { x: 0, y: 0 });
    assert_eq!(val, None);
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn dense_number_grid_2d_update_test() {
    let mut grid: DenseNumberGrid2D<u16> = DenseNumberGrid2D::new(WIDTH, HEIGHT);
    grid.set_value_location(1, &Int2D { x: 0, y: 0 });
    grid.update();
    assert_eq!(grid.get_value(&Int2D { x: 0, y: 0 }), Some(1));
    assert_eq!(grid.get_value_unbuffered(&Int2D { x: 0, y: 0 }), None);
}
