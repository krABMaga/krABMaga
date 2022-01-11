#[cfg(test)]

static HEIGHT:i32 = 10;
static WIDTH:i32 = 10;

use rust_ab::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use rust_ab::engine::fields::field::Field;
use rust_ab::engine::fields::grid_option::GridOption;
use rust_ab::engine::location::Int2D;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm", feature = "parallel")))]
#[test]
fn dense_number_grid_2d(){
    let mut grid: DenseNumberGrid2D<u16> = DenseNumberGrid2D::new(WIDTH, HEIGHT);

    let all = grid.get_empty_bags();
    assert_eq!((HEIGHT*WIDTH) as usize, all.len());


    for i in 0..10 {
        for j in 0..10 {
            let loc = Int2D { x: i, y: j };
            grid.set_value_location(0, &loc);
        }
    }

    grid.lazy_update();

    grid.apply_to_all_values(
        |_value| {
            1
        },
        GridOption::WRITE,
    );

    grid.lazy_update();

    grid.apply_to_all_values(
        |value| {
            let val = *value;
            val + 1
        },
        GridOption::READWRITE,
    );

    grid.lazy_update();

    
    grid.apply_to_all_values(
        |value| {
            *value
        },
        GridOption::READ,
    );

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
            grid.set_value_location((i*j) as u16, &loc);
        }   
    }

    grid.lazy_update();

    grid.iter_values(
        |loc, val| {
            let value = grid.get_value(&loc).unwrap();
            assert_eq!(*val, value);
            assert_eq!((loc.x*loc.y) as u16, value);
        }
    );

    let none = grid.get_empty_bags();
    assert_eq!(0, none.len());


}
