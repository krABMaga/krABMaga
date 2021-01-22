use crate::model::static_objects::StaticObjectType;
use rust_ab::engine::field::object_grid_2d::Grid2D;

/// A grid of obstacles, static dense objects that block the ants' movement on that particular cell.
pub struct ObstaclesGrid {
    pub grid: Grid2D<StaticObjectType>,
}

impl ObstaclesGrid {
    pub fn new(width: i64, height: i64) -> ObstaclesGrid {
        ObstaclesGrid {
            grid: Grid2D::new(width, height),
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
    }
}
