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
    crate::model::flockers::bird::Bird,
    krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D,
    krabmaga::engine::fields::field::Field, krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::location::Int2D, krabmaga::engine::location::Real2D,
};

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn dense_object_grid_2d_bags() {
    let mut grid: DenseGrid2D<Bird> = DenseGrid2D::new(WIDTH, HEIGHT);

    let vec = grid.get_empty_bags();
    assert_eq!(vec.len(), 100);

    let loc = grid.get_random_empty_bag();

    assert!(None != loc);
    let loc = loc.unwrap();

    grid.set_object_location(
        Bird::new(0, Real2D { x: 0., y: 0. }, Real2D { x: 0., y: 0. }),
        &loc,
    );

    let loc2 = grid.get_location_unbuffered(&Bird::new(
        0,
        Real2D { x: 0., y: 0. },
        Real2D { x: 0., y: 0. },
    ));
    assert!(loc2.is_some());
    let loc2 = loc2.unwrap();
    assert_eq!(loc.x, loc2.x);
    assert_eq!(loc.y, loc2.y);

    let loc2 = grid.get_location_unbuffered(&Bird::new(
        1,
        Real2D { x: 0., y: 0. },
        Real2D { x: 0., y: 0. },
    ));
    assert!(loc2.is_none());

    grid.lazy_update();

    let loc2 = grid.get_location(&Bird::new(
        0,
        Real2D { x: 0., y: 0. },
        Real2D { x: 0., y: 0. },
    ));
    assert!(loc2.is_some());
    let loc2 = loc2.unwrap();
    assert_eq!(loc.x, loc2.x);
    assert_eq!(loc.y, loc2.y);

    let loc2 = grid.get_location(&Bird::new(
        1,
        Real2D { x: 0., y: 0. },
        Real2D { x: 0., y: 0. },
    ));
    assert!(loc2.is_none());

    let vec = grid.get_empty_bags();
    assert_eq!(vec.len(), 99);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let loc = Int2D { x: i, y: j };
            grid.set_object_location(
                Bird::new(
                    (i * HEIGHT + j) as u32,
                    Real2D { x: 0., y: 0. },
                    Real2D { x: 0., y: 0. },
                ),
                &loc,
            );
        }
    }

    grid.lazy_update();
    let vec = grid.get_empty_bags();
    assert_eq!(vec.len(), 0);
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn dense_object_grid_2d_apply() {
    let mut grid: DenseGrid2D<Bird> = DenseGrid2D::new(WIDTH, HEIGHT);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let loc = Int2D { x: i, y: j };
            grid.set_object_location(
                Bird::new(
                    (i * HEIGHT + j) as u32,
                    Real2D { x: 0., y: 0. },
                    Real2D { x: 0., y: 0. },
                ),
                &loc,
            );
        }
    }
    grid.iter_objects_unbuffered(|loc, val| {
        let value = grid.get_objects_unbuffered(loc);
        assert!(None != value);
        assert_eq!(value.unwrap()[0].id, val.id);
    });
    grid.lazy_update();
    grid.iter_objects(|loc, val| {
        let value = grid.get_objects(loc);
        assert!(None != value);
        assert_eq!(value.unwrap()[0].id, val.id);
    });

    grid.apply_to_all_values(
        |_index, bird| {
            let mut b = *bird;
            b.flag = true;
            Some(b)
        },
        GridOption::WRITE,
    );

    grid.iter_objects_unbuffered(|_loc, bird| {
        assert!(bird.flag);
    });

    //------
    grid.apply_to_all_values(
        |_index, bird| {
            let mut b = *bird;
            b.flag = false;
            Some(b)
        },
        GridOption::READ,
    );
    grid.iter_objects(|_loc, bird| {
        assert!(!bird.flag);
    });

    //------
    grid.apply_to_all_values(
        |_index, bird| {
            let mut b = *bird;
            b.flag = true;
            Some(b)
        },
        GridOption::READWRITE,
    );
    grid.lazy_update();
    grid.iter_objects(|_loc, bird| {
        assert!(bird.flag);
    });
}
