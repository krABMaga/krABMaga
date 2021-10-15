use crate::engine::location::Int2D;
use crate::utils::dbdashmap::DBDashMap;

/// A crude implementation of a matrix based grid, for quick access of specific positions.
pub struct NumberGrid2D<T: Copy + Clone> {
    pub locs: DBDashMap<Int2D, T>,
    pub width: i32,
    pub height: i32,
}

impl<T: Copy + Clone> NumberGrid2D<T> {
    /// Initializes a NumberGrid2D that wraps a width * height matrix, with values of type Option<T>.
    pub fn new(width: i32, height: i32) -> NumberGrid2D<T> {
        NumberGrid2D {
            locs: DBDashMap::new(),
            width,
            height,
        }
    }

    /// Fetches the value contained within a cell of the matrix.
    ///
    /// None if the cell's empty, Some(T) otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_ab::engine::field::number_grid_2d::NumberGrid2D;
    /// # use rust_ab::engine::location::Int2D;
    ///
    /// let mut simple_grid = NumberGrid2D::new(10, 10);
    /// let value = 5;
    /// let loc = Int2D{x: 2, y: 2};
    /// simple_grid.set_value_at_pos(&loc, value);
    /// simple_grid.update();
    /// let cell_value = simple_grid.get_value_at_pos(&loc);
    /// assert_eq!(cell_value, Some(&5));
    /// ```
    pub fn get_value_at_pos(&self, pos: &Int2D) -> Option<&T> {
        self.locs.get(pos)
    }

    /// Sets the value of a matrix's cell to a copy of T.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_ab::engine::field::number_grid_2d::NumberGrid2D;
    /// # use rust_ab::engine::location::Int2D;
    ///
    /// let mut simple_grid = NumberGrid2D::new(10, 10);
    /// let value = 5;
    /// let loc = Int2D{x: 2, y: 2};
    /// simple_grid.set_value_at_pos(&loc, value);
    /// simple_grid.update();
    /// let cell_value = simple_grid.get_value_at_pos(&loc);
    /// assert_eq!(cell_value, Some(&5));
    /// ```
    pub fn set_value_at_pos(&self, pos: &Int2D, value: T) {
        self.locs.insert(*pos, value);
    }

    pub fn update(&mut self) {
        self.locs.update();
    }

    pub fn lazy_update(&mut self) {
        self.locs.lazy_update();
    }

}

/*
impl<T: Copy + Clone + PartialOrd> NumberGrid2D<T> {
    /// Returns a copy of the minimum T value, wrapped in an Option, contained within the matrix.
    ///
    /// Returns None if the matrix is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_ab::engine::field::number_grid_2d::NumberGrid2D;
    /// # use rust_ab::engine::location::Int2D;
    ///
    /// let mut simple_grid = NumberGrid2D::new(10, 10);
    /// for x in 0..10 {
    ///     for y in 0..10 {
    ///         simple_grid.set_value_at_pos(&Int2D{x, y}, x+y);
    ///     }
    /// }
    /// assert_eq!(simple_grid.min(), Some(0));
    /// ```
    pub fn min(&self) -> Option<T> {
        let mut min: Option<T> = None;
        for i in self.locs.iter() {
            if let Some(pos) = i {
                if let Some(actual_min) = min {
                    if *pos < actual_min {
                        min = Some(*pos);
                    }
                } else {
                    min = Some(*pos);
                }
            }
        }
        min
    }

    /// Returns a copy of the maximum T value, wrapped in an Option, contained within the matrix.
    ///
    /// Returns None if the matrix is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_ab::engine::field::number_grid_2d::NumberGrid2D;
    /// # use rust_ab::engine::location::Int2D;
    ///
    /// let mut simple_grid = NumberGrid2D::new(10, 10);
    /// for x in 0..10 {
    ///     for y in 0..10 {
    ///         simple_grid.set_value_at_pos(&Int2D{x, y}, x+y);
    ///     }
    /// }
    /// assert_eq!(simple_grid.max(), Some(18));
    /// ```
    pub fn max(&self) -> Option<T> {
        let mut max: Option<T> = None;
        for i in self.locs.iter() {
            if let Some(pos) = i {
                if let Some(actual_min) = max {
                    if *pos > actual_min {
                        max = Some(*pos);
                    }
                } else {
                    max = Some(*pos);
                }
            }
        }
        max
    }
}

impl NumberGrid2D<f32> {
    /// Multiply all the grid cells by a specific value. If the value becomes smaller than
    /// round_if_lower, round it to round_to.
    pub fn multiply_with_rounding(&mut self, value: f32, round_if_lower: f32, round_to: f32) {
        for i in self.locs.iter_mut() {
            if let Some(val) = i {
                *val *= value;
                if *val < round_if_lower {
                    *val = round_to;
                }
            }
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use crate::engine::field::number_grid_2d::NumberGrid2D;
    use crate::engine::location::Int2D;

    #[test]
    fn simple_grid_2d() {
        let mut grid = NumberGrid2D::<i32>::new(10, 10);
        let pos = Int2D { x: 2, y: 3 };
        let pos2 = Int2D { x: 4, y: 5 };
        let pos3 = Int2D { x: 5, y: 5 };
        grid.set_value_at_pos(&pos, 5);
        grid.set_value_at_pos(&pos2, 10);
        grid.update();
        let val = grid.get_value_at_pos(&pos);
        assert_eq!(val, Some(&5));

        let val = grid.get_value_at_pos(&pos2);
        assert_eq!(val, Some(&10));

        let val = grid.get_value_at_pos(&pos3);
        assert_eq!(val, None);
    }
}
