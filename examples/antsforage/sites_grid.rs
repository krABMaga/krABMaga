use abm::simple_grid_2d::SimpleGrid2D;

use crate::static_objects::StaticObjectType;

/// Represents static point of interests, such as nests or food sources.
pub struct SitesGrid {
    pub grid: SimpleGrid2D<StaticObjectType>,
}

impl SitesGrid {
    pub fn new(width: i64, height: i64) -> SitesGrid {
        SitesGrid {
            grid: SimpleGrid2D::new(width, height),
        }
    }
}
