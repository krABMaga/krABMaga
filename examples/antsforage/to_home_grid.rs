use abm::simple_grid_2d::SimpleGrid2D;

const LOW_PHEROMONE: f64 = 0.00000000000001;

/// Represents home pheromones. Higher f64 means more concentrated pheromone.
pub struct ToHomeGrid {
    pub grid: SimpleGrid2D<f64>,
}

impl ToHomeGrid {
    pub fn new(width: i64, height: i64) -> ToHomeGrid {
        ToHomeGrid {
            grid: SimpleGrid2D::new(width, height),
        }
    }

    /// Multiply the inner grid by value, rounding the pheromone to zero if it becomes extremely small
    /// to avoid a performance hit with extremely small amount of pheromones.
    pub fn multiply(&mut self, value: f64) {
        self.grid.multiply_with_rounding(value, LOW_PHEROMONE, 0.);
    }
}
