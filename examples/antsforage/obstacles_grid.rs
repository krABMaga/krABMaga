use abm::simple_grid_2d::SimpleGrid2D;

use crate::static_objects::StaticObjectType;

/// A grid of obstacles, static dense objects that block the ants' movement on that particular cell.
pub struct ObstaclesGrid {
    pub grid: SimpleGrid2D<StaticObjectType>,
}

impl ObstaclesGrid {
    pub fn new(width: i64, height: i64) -> ObstaclesGrid {
        ObstaclesGrid {
            grid: SimpleGrid2D::new(width, height),
        }
    }
}
