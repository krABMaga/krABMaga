use crate::location::Int2D;

// A crude implementation of a matrix based grid, for easy access of specific positions.
pub struct SimpleGrid2D<T: Copy + Clone> {
    pub locs: Vec<Vec<Option<T>>>,
    pub width: i64,
    pub height: i64,
}

impl<T: Copy + Clone> SimpleGrid2D<T> {
    pub fn new(width: i64, height: i64) -> SimpleGrid2D<T> {
        SimpleGrid2D {
            locs: vec![vec![None; height as usize]; width as usize],
            width,
            height,
        }
    }

    pub fn get_value_at_pos(&self, pos: &Int2D) -> Option<T> {
        return self.locs[pos.x as usize][pos.y as usize];
    }

    pub fn set_value_at_pos(&mut self, pos: &Int2D, value: T) {
        self.locs[pos.x as usize][pos.y as usize] = Some(value);
    }
}

impl<T: Copy + Clone + PartialOrd> SimpleGrid2D<T> {
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
