use crate::model::static_objects::StaticObjectType;
use rust_ab::engine::field::object_grid_2d::Grid2D;

/// Represents static point of interests, such as nests or food sources.
pub struct SitesGrid {
    pub grid: Grid2D<StaticObjectType>,
}

impl SitesGrid {
    pub fn new(width: i64, height: i64) -> SitesGrid {
        SitesGrid {
            grid: Grid2D::new(width, height),
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
    }
}
