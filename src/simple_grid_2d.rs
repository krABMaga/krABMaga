use crate::{location::{Int2D}};


// A crude implementation of a matrix based grid, for easy access of specific positions.
pub struct SimpleGrid2D<T: Copy + Clone> {
	pub locs: Vec<Vec<Option<T>>>,
	pub width: i64,
	pub height: i64,
}


impl<T: Copy + Clone> SimpleGrid2D<T> {
	pub fn new(width: i64, height: i64) -> SimpleGrid2D<T> {
		SimpleGrid2D {
			locs: vec![vec![None;height as usize]; width as usize],
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