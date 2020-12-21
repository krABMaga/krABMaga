use abm::simple_grid_2d::SimpleGrid2D;

/// Extremely low pheromone, under which the value gets rounded to 0
const LOW_PHEROMONE: f64 = 0.00000000000001;

/// Represents food pheromones. Higher f64 value means more concentrated pheromone.
pub struct ToFoodGrid {
    pub grid: SimpleGrid2D<f64>,
}

impl ToFoodGrid {
    pub fn new(width: i64, height: i64) -> ToFoodGrid {
        ToFoodGrid {
            grid: SimpleGrid2D::new(width, height),
        }
    }

    /// Multiply the inner grid by value, rounding the pheromone to zero if it becomes extremely small
    /// to avoid a performance hit with extremely small amount of pheromones.
    pub fn multiply(&mut self, value: f64) {
        self.grid.multiply_with_rounding(value, LOW_PHEROMONE, 0.);
    }
}
