use abm::{simple_grid_2d::SimpleGrid2D};
use amethyst::ecs::world::Index;

// Represents home pheromones. Higher f64 = more concentrated pheromone
pub struct ToHomeGrid {
    pub grid: SimpleGrid2D<(Index, f64)>
}

impl ToHomeGrid {
    pub fn new(width: i64, height: i64) -> ToHomeGrid {
        ToHomeGrid {
            grid: SimpleGrid2D::new(width, height)
        }
    }
}