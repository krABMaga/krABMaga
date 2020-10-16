use abm::grid_2d::Grid2D;

use crate::agent_adapter::AgentAdapter;


// Represents the main field with ants.
pub struct AntsGrid {
	pub grid: Grid2D<AgentAdapter>
}

impl AntsGrid {
	pub fn new(width: i64, height: i64) -> AntsGrid {
		AntsGrid {
			grid: Grid2D::new(width, height)
		}
	}
}