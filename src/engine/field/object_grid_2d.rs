use std::hash::Hash;

use crate::location::Int2D;
use hashbrown::HashMap;

/// A crude implementation of a grid that wraps a HashMap, with agents as keys and their locations as values.
pub struct Grid2D<A: Eq + Hash + Clone + Copy> {
    pub locs: HashMap<A, Int2D>,
    pub width: i64,
    pub height: i64,
}

impl<A: Eq + Hash + Clone + Copy> Grid2D<A> {
    /// Initializes a Grid2D with a specied capacity of width * height.
    pub fn new(width: i64, height: i64) -> Grid2D<A> {
        Grid2D {
            locs: HashMap::with_capacity((width * height) as usize),
            width,
            height,
        }
    }

    /// Inserts the agent in the grid, with the agent itself as key and its position as value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use abm::agent::Agent;
    /// use abm::grid_2d::Grid2D;
    /// use abm::location::Int2D;
    /// #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    /// struct A {};
    /// impl Agent for A {fn step(&mut self) {
    ///         println!("Stepping!");
    ///     }
    /// }
    /// let mut grid = Grid2D::new(10,10);
    /// let mut agent = A{};
    /// let loc = Int2D{x: 2, y: 2};
    /// grid.set_object_location(&mut agent, &loc);
    /// assert!(grid.get_object_location(&agent) == Some(&loc));
    /// ```
    pub fn set_object_location(&mut self, agent: &mut A, new_pos: &Int2D) {
        let mut agent_loc = self.locs.entry(*agent).or_insert(*new_pos);
        agent_loc.x = new_pos.x;
        agent_loc.y = new_pos.y;
    }

    /// Fetches the position of the agent in the grid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use abm::agent::Agent;
    /// use abm::grid_2d::Grid2D;
    /// use abm::location::Int2D;
    /// #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    /// struct A {};
    /// impl Agent for A {fn step(&mut self) {
    ///         println!("Stepping!");
    ///     }
    /// }
    /// let mut grid = Grid2D::new(10,10);
    /// let mut agent = A{};
    /// let loc = Int2D{x: 2, y: 2};
    /// grid.set_object_location(&mut agent, &loc);
    /// assert!(grid.get_object_location(&agent) == Some(&loc));
    /// ```
    pub fn get_object_location(&self, agent: &A) -> Option<&Int2D> {
        self.locs.get(agent)
    }
}
