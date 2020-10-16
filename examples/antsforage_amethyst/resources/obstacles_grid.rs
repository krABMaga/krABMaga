use abm::simple_grid_2d::SimpleGrid2D;

use crate::static_object::StaticObjectType;

// Grid representing simple static obstacles.
pub struct ObstaclesGrid {
	pub grid: SimpleGrid2D<StaticObjectType>
}

impl ObstaclesGrid {
	pub fn new(width: i64, height: i64) -> ObstaclesGrid {
		ObstaclesGrid {
			grid: SimpleGrid2D::new(width, height)
		}
	}
}