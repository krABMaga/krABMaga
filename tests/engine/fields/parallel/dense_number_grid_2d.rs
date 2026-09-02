#[cfg(test)]
#[cfg(any(feature = "parallel"))]
static HEIGHT: i32 = 10;
#[cfg(any(feature = "parallel"))]
static WIDTH: i32 = 10;

#[cfg(any(feature = "parallel"))]
use {
    krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
    krabmaga::engine::fields::field::Field, krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::location::Int2D,
};

#[cfg(any(feature = "parallel"))]
#[test]
fn dense_number_grid_2d() {
    let mut grid: DenseNumberGrid2D<u16> = DenseNumberGrid2D::new(WIDTH, HEIGHT);

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location(0, &loc);
        }
    }

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
            assert_eq!(val, 3);
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            let val = grid.get_value(&loc).unwrap();
            assert_eq!(val, 2);
        }
    }

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
            assert_eq!(val, (i * j) as u16);
        }
    }
}
