use crate::EVAPORATION;
use rust_ab::engine::field::number_grid_2d::NumberGrid2D;

/// Extremely low pheromone, under which the value gets rounded to 0
const LOW_PHEROMONE: f64 = 0.00000000000001;

/// Represents food pheromones. Higher f64 value means more concentrated pheromone.
pub struct ToFoodGrid {
    pub grid: NumberGrid2D<f64>,
}

impl ToFoodGrid {
    pub fn new(width: i64, height: i64) -> ToFoodGrid {
        ToFoodGrid {
            grid: NumberGrid2D::new(width, height),
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
        self.grid.locs.apply_to_all_values(|val| {
            let new_val = val * EVAPORATION;
            if new_val < LOW_PHEROMONE {
                0.
            } else {
                new_val
            }
        })
    }
}
