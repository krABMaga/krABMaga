#[cfg(test)]
#[cfg(any(feature = "parallel"))]
static HEIGHT: i32 = 10;
#[cfg(any(feature = "parallel"))]
static WIDTH: i32 = 10;

#[cfg(any(feature = "parallel"))]
use {
    crate::model::flockers::bird::Bird,
    krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D,
    krabmaga::engine::fields::field::Field, krabmaga::engine::fields::grid_option::GridOption,
    krabmaga::engine::location::Int2D, krabmaga::engine::location::Real2D, rand::Rng,
};

#[cfg(any(feature = "parallel"))]
#[test]
fn dense_object_grid_2d_gets() {
    let mut grid: DenseGrid2D<Bird> = DenseGrid2D::new(WIDTH, HEIGHT);
    let mut rng = rand::rng();

    let loc = Int2D {
        x: rng.random_range(0..WIDTH),
        y: rng.random_range(0..HEIGHT),
    };
    let no_bird = grid.get_objects(&loc);
    assert!(None == no_bird);

    let bird1 = Bird::new(0, Real2D { x: 0., y: 0. }, Real2D { x: 0., y: 0. });
    grid.set_object_location(bird1, &loc);

    let bird = grid.get_objects_unbuffered(&loc);
    assert!(None != bird);
    assert_eq!(bird.unwrap()[0].id, bird1.id);
    let no_bird = grid.get_objects(&loc);
    assert!(None == no_bird);
    let no_bird = grid.get(&bird1);
    assert!(None == no_bird);

    let get_loc = grid.get_location_unbuffered(bird1);
    assert!(None != get_loc);
    assert_eq!(get_loc.unwrap().x, loc.x);
    assert_eq!(get_loc.unwrap().y, loc.y);
    let no_loc = grid.get_location(bird1);
    assert!(None == no_loc);
    grid.update();

    let bird = grid.get_objects(&loc);
    assert!(None != bird);
    assert_eq!(bird.unwrap()[0].id, bird1.id);
    let bird = grid.get(&bird1);
    assert!(None != bird);
    assert_eq!(bird.unwrap().id, bird1.id);

    //-----
    grid.remove_object(&bird1);
    grid.lazy_update();

    let no_bird = grid.get_objects_unbuffered(&loc);
    assert!(None == no_bird);
    let no_bird = grid.get_objects(&loc);
    assert!(None == no_bird);
    let no_bird = grid.get(&bird1);
    assert!(None == no_bird);
}

#[cfg(any(feature = "parallel"))]
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

    grid.lazy_update();

    grid.apply_to_all_values(
        |_index, bird| {
            let mut b = *bird;
            b.flag = true;
            Some(b)
        },
        GridOption::WRITE,
    );

    grid.lazy_update();
    grid.iter_objects(|_loc, bird| {
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
