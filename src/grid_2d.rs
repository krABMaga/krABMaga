use std::hash::Hash;

use crate::location::{Int2D};
use hashbrown::HashMap;


//TODO: generalize over Location2D
// A crude implementation of a grid, with agents as keys and their locations as values.
pub struct Grid2D<A: Eq + Hash + Clone + Copy> {
	pub locs: HashMap<A, Int2D>,
	pub width: i64,
	pub height: i64,
}


impl<A: Eq + Hash + Clone + Copy> Grid2D<A> {
	// TODO: generalize width and height?
	pub fn new(width: i64, height: i64) -> Grid2D<A> {
		Grid2D {
			locs: HashMap::new(), // TODO: pre-allocate memory?
			width,
			height,
		}
	}

	pub fn set_object_location(&mut self, agent: &mut A, new_pos: Int2D) {
		let mut agent_loc = self.locs
			.entry(*agent)
			.or_insert(new_pos);
		agent_loc.x = new_pos.x;
		agent_loc.y = new_pos.y;
	}
}