use crate::EVAPORATION;
use rust_ab::engine::field::number_grid_2d::NumberGrid2D;

const LOW_PHEROMONE: f64 = 0.00000000000001;

/// Represents home pheromones. Higher f64 means more concentrated pheromone.
pub struct ToHomeGrid {
    pub grid: NumberGrid2D<f64>,
}

impl ToHomeGrid {
    pub fn new(width: i64, height: i64) -> ToHomeGrid {
        ToHomeGrid {
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
