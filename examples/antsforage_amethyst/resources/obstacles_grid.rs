use abm::simple_grid_2d::SimpleGrid2D;

use crate::static_object::StaticObject;

// Grid representing simple static obstacles.
pub struct ObstaclesGrid {
    pub grid: SimpleGrid2D<StaticObject>
}

impl ObstaclesGrid {
    pub fn new(width: i64, height: i64) -> ObstaclesGrid {
        ObstaclesGrid {
            grid: SimpleGrid2D::new(width, height)
        }
    }
}