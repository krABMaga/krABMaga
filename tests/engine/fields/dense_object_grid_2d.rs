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

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn dense_object_grid_2d_missing_branches() {
    let mut grid: DenseGrid2D<Bird> = DenseGrid2D::new(2, 2);

    let bird_a = Bird::new(1, Real2D { x: 0., y: 0. }, Real2D { x: 0., y: 0. });
    let bird_b = Bird::new(2, Real2D { x: 0., y: 0. }, Real2D { x: 0., y: 0. });
    let loc = Int2D { x: 0, y: 0 };
    let empty_loc = Int2D { x: 1, y: 1 };

    // Cover retain path in set_object_location and remove_object_location.
    grid.set_object_location(bird_a, &loc);
    grid.set_object_location(bird_a, &loc);
    grid.set_object_location(bird_b, &loc);
    grid.remove_object_location(bird_a, &loc);
    grid.remove_object_location(bird_b, &empty_loc);
    let objs = grid.get_objects_unbuffered(&loc).unwrap();
    assert_eq!(objs.len(), 1);
    assert_eq!(objs[0].id, bird_b.id);

    // Cover get_objects_unbuffered None branch.
    assert!(grid.get_objects_unbuffered(&empty_loc).is_none());

    // Update read state without swapping and cover get_objects None branch.
    let mut grid_update: DenseGrid2D<Bird> = DenseGrid2D::new(2, 2);
    grid_update.set_object_location(bird_a, &loc);
    grid_update.update();
    assert!(grid_update.get_objects(&empty_loc).is_none());

    // Cover READ and WRITE Some/None branches.
    let mut grid_apply: DenseGrid2D<Bird> = DenseGrid2D::new(2, 2);
    grid_apply.set_object_location(bird_a, &loc);
    grid_apply.set_object_location(bird_b, &loc);
    grid_apply.lazy_update();
    grid_apply.apply_to_all_values(
        |_loc, bird| {
            if bird.id == bird_b.id {
                None
            } else {
                let mut b = *bird;
                b.flag = true;
                Some(b)
            }
        },
        GridOption::READ,
    );

    grid_apply.apply_to_all_values(
        |_loc, bird| {
            if bird.id == bird_a.id {
                None
            } else {
                let mut b = *bird;
                b.flag = false;
                Some(b)
            }
        },
        GridOption::WRITE,
    );

    // Hit READWRITE write-branch Some/None.
    grid_apply.apply_to_all_values(
        |_loc, bird| {
            if bird.id == bird_a.id {
                None
            } else {
                let mut b = *bird;
                b.flag = true;
                Some(b)
            }
        },
        GridOption::READWRITE,
    );

    // Ensure write bag is empty then hit READWRITE read-branch.
    let mut grid_read: DenseGrid2D<Bird> = DenseGrid2D::new(2, 2);
    grid_read.set_object_location(bird_a, &loc);
    grid_read.lazy_update();
    grid_read.apply_to_all_values(
        |_loc, bird| {
            let mut b = *bird;
            b.flag = true;
            Some(b)
        },
        GridOption::READWRITE,
    );
    assert!(grid_read.get_objects_unbuffered(&loc).is_some());

    // Cover READWRITE contains false branch by returning duplicates.
    let mut grid_contains: DenseGrid2D<Bird> = DenseGrid2D::new(2, 2);
    grid_contains.set_object_location(bird_a, &loc);
    grid_contains.set_object_location(bird_b, &loc);
    grid_contains.lazy_update();
    grid_contains.apply_to_all_values(|_loc, _bird| Some(bird_a), GridOption::READWRITE);

    // Cover READWRITE None branch in the read path.
    let mut grid_none: DenseGrid2D<Bird> = DenseGrid2D::new(2, 2);
    grid_none.set_object_location(bird_a, &loc);
    grid_none.set_object_location(bird_b, &loc);
    grid_none.lazy_update();
    grid_none.apply_to_all_values(
        |_loc, bird| {
            if bird.id == bird_a.id {
                None
            } else {
                Some(*bird)
            }
        },
        GridOption::READWRITE,
    );

    // Cover get_random_empty_bag None branch when read is full.
    grid.lazy_update();
    for i in 0..2 {
        for j in 0..2 {
            let fill_loc = Int2D { x: i, y: j };
            let bird = Bird::new(
                (i * 10 + j) as u32,
                Real2D { x: 0., y: 0. },
                Real2D { x: 0., y: 0. },
            );
            grid.set_object_location(bird, &fill_loc);
        }
    }
    grid.lazy_update();
    assert!(grid.get_random_empty_bag().is_none());

    // Cover iter_objects/iter_objects_unbuffered empty-branch.
    let empty_grid: DenseGrid2D<Bird> = DenseGrid2D::new(1, 1);
    let count = std::cell::Cell::new(0);
    empty_grid.iter_objects(|_, _| count.set(count.get() + 1));
    empty_grid.iter_objects_unbuffered(|_, _| count.set(count.get() + 1));
    assert_eq!(count.get(), 0);
}
