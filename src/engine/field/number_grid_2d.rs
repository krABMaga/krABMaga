use crate::location::Int2D;

/// A crude implementation of a matrix based grid, for quick access of specific positions.
pub struct NumberGrid2D<T: Copy + Clone> {
    pub locs: Vec<Vec<Option<T>>>,
    pub width: i64,
    pub height: i64,
}

impl<T: Copy + Clone> SimpleGrid2D<T> {
    /// Initializes a SimpleGrid2D that wraps a width * height matrix, with values of type Option<T>.
    pub fn new(width: i64, height: i64) -> SimpleGrid2D<T> {
        SimpleGrid2D {
            locs: vec![vec![None; height as usize]; width as usize],
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
    /// # use abm::simple_grid_2d::SimpleGrid2D;
    /// # use abm::location::Int2D;
    ///
    /// let mut simple_grid = SimpleGrid2D::new(10, 10);
    /// let value = 5;
    /// let loc = Int2D{x: 2, y: 2};
    /// simple_grid.set_value_at_pos(&loc, value);
    /// let cell_value = simple_grid.get_value_at_pos(&loc);
    /// assert_eq!(cell_value, Some(5));
    /// ```
    pub fn get_value_at_pos(&self, pos: &Int2D) -> Option<T> {
        return self.locs[pos.x as usize][pos.y as usize];
    }

    /// Sets the value of a matrix's cell to a copy of T.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use abm::simple_grid_2d::SimpleGrid2D;
    /// # use abm::location::Int2D;
    ///
    /// let mut simple_grid = SimpleGrid2D::new(10, 10);
    /// let value = 5;
    /// let loc = Int2D{x: 2, y: 2};
    /// simple_grid.set_value_at_pos(&loc, value);
    /// let cell_value = simple_grid.get_value_at_pos(&loc);
    /// assert_eq!(cell_value, Some(5));
    /// ```
    pub fn set_value_at_pos(&mut self, pos: &Int2D, value: T) {
        self.locs[pos.x as usize][pos.y as usize] = Some(value);
    }
}

impl<T: Copy + Clone + PartialOrd> SimpleGrid2D<T> {
    /// Returns a copy of the minimum T value, wrapped in an Option, contained within the matrix.
    ///
    /// Returns None if the matrix is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use abm::simple_grid_2d::SimpleGrid2D;
    /// # use abm::location::Int2D;
    ///
    /// let mut simple_grid = SimpleGrid2D::new(10, 10);
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
            for j in i.iter() {
                if let Some(pos) = j {
                    if let Some(actual_min) = min {
                        if *pos < actual_min {
                            min = Some(*pos);
                        }
                    } else {
                        min = Some(*pos);
                    }
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
    /// # use abm::simple_grid_2d::SimpleGrid2D;
    /// # use abm::location::Int2D;
    ///
    /// let mut simple_grid = SimpleGrid2D::new(10, 10);
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
            for j in i.iter() {
                if let Some(pos) = j {
                    if let Some(actual_min) = max {
                        if *pos > actual_min {
                            max = Some(*pos);
                        }
                    } else {
                        max = Some(*pos);
                    }
                }
            }
        }
        max
    }
}

impl SimpleGrid2D<f64> {
    /// Multiply all the grid cells by a specific value. If the value becomes smaller than
    /// round_if_lower, round it to round_to.
    pub fn multiply_with_rounding(&mut self, value: f64, round_if_lower: f64, round_to: f64) {
        for i in self.locs.iter_mut() {
            for j in i.iter_mut() {
                if let Some(val) = j {
                    *val *= value;
                    if *val < round_if_lower {
                        *val = round_to;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Int2D;
    use crate::simple_grid_2d::SimpleGrid2D;

    #[test]
    fn simple_grid_2d() {
        let mut grid = SimpleGrid2D::<i64>::new(10, 10);
        let pos = Int2D { x: 2, y: 3 };
        let pos2 = Int2D { x: 4, y: 5 };
        grid.set_value_at_pos(&pos, 5);
        grid.set_value_at_pos(&pos2, 10);
        let val = grid.get_value_at_pos(&pos);
        assert_eq!(val, Some(5));
        assert_eq!(grid.min(), Some(5));
        assert_eq!(grid.max(), Some(10));
    }
}
