use std::hash::Hash;

use crate::engine::location::Int2D;
use crate::utils::dbdashmap::DBDashMap;

/// A crude implementation of a sparse double buffered grid.
pub struct Grid2D<A: Eq + Hash + Clone + Copy> {
    pub locs: DBDashMap<A, Int2D>,
    pub locs_inversed: DBDashMap<Int2D, Vec<A>>, // TODO consider using a linked list instead of a vec?
    pub width: i64,
    pub height: i64,
}

impl<A: Eq + Hash + Clone + Copy> Grid2D<A> {
    /// Initializes a Grid2D with a specific capacity of width * height.
    pub fn new(width: i64, height: i64) -> Grid2D<A> {
        Grid2D {
            locs: DBDashMap::with_capacity((width * height) as usize),
            locs_inversed: DBDashMap::with_capacity((width * height) as usize),
            width,
            height,
        }
    }

    /// Inserts the agent in the grid, with the agent itself as key and its position as value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_ab::engine::agent::Agent;
    /// use rust_ab::engine::field::object_grid_2d::Grid2D;
    /// use rust_ab::engine::location::Int2D;
    /// use rust_ab::engine::state::State;
    ///
    /// #[derive(Copy, Clone, Eq, Hash, Debug)]
    /// struct MyObject {
    ///     pub id: i32,
    ///     pub loc: Int2D,
    /// }
    ///
    /// impl PartialEq for MyObject {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.id == other.id
    ///     }
    /// }
    ///
    /// let mut grid = Grid2D::new(10,10);
    /// let loc = Int2D{x: 2, y: 2};
    /// let mut obj = MyObject{id: 1, loc};
    /// grid.set_object_location(obj, &loc);
    /// grid.update();
    /// assert_eq!(grid.get_object_location(obj), Some(&loc));
    /// ```
    pub fn set_object_location(&self, agent: A, new_pos: &Int2D) {
        self.remove_object(&agent);

        let existing_elements_vec = self.locs_inversed.get_mut(new_pos);

        match existing_elements_vec {
            Some(mut cell_ref) => {
                cell_ref.value_mut().push(agent);
            }
            None => {
                self.locs_inversed.insert(*new_pos, vec![agent]);
            }
        };

        self.locs.insert(agent, *new_pos);
    }

    /// Removes an agent from the grid. As the other operations, this method acts on the write buffer.
    /// Therefore, the change will not be visible until the update method is called to sync the buffers.
    /// If the agent passed is already absent from the grid, nothing will happen.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_ab::engine::agent::Agent;
    /// use rust_ab::engine::field::object_grid_2d::Grid2D;
    /// use rust_ab::engine::location::Int2D;
    /// use rust_ab::engine::state::State;
    ///
    /// #[derive(Copy, Clone, Eq, Hash, Debug)]
    /// struct MyObject {
    ///     pub id: i32,
    ///     pub loc: Int2D,
    /// }
    ///
    /// impl PartialEq for MyObject {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.id == other.id
    ///     }
    /// }
    ///
    /// let mut grid = Grid2D::new(10,10);
    /// let loc = Int2D{x: 2, y: 2};
    /// let mut obj = MyObject{id: 1, loc};
    /// grid.set_object_location(obj, &loc);
    /// grid.update();
    /// assert_eq!(grid.get_object_location(obj), Some(&loc));
    /// grid.remove_object(&obj);
    /// grid.update();
    /// assert_eq!(grid.get_object_location(obj), None);
    /// ```
    pub fn remove_object(&self, agent: &A) {
        if let Some(old_loc) = self.locs.get(agent) {
            self.locs_inversed
                .get_mut(old_loc)
                .unwrap()
                .value_mut()
                .retain(|&x| x != *agent);
        }

        self.locs.remove(agent);
    }

    /// Fetches the copy of MyObject stored within the grid. This is necessary due to the grid being
    /// the source of truth of the simulation. Instead of relying on a local copy of an agent, the
    /// developer should fetch the up to date copy of it through this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_ab::engine::agent::Agent;
    /// use rust_ab::engine::field::object_grid_2d::Grid2D;
    /// use rust_ab::engine::location::Int2D;
    /// use rust_ab::engine::state::State;
    ///
    /// #[derive(Copy, Clone, Eq, Hash, Debug)]
    /// struct MyObject {
    ///     pub id: i32,
    ///     pub loc: Int2D,
    /// }
    ///
    /// impl PartialEq for MyObject {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.id == other.id
    ///     }
    /// }
    ///
    /// let mut grid = Grid2D::new(10,10);
    /// let loc = Int2D{x: 2, y: 2};
    /// let mut obj = MyObject{id: 1, loc};
    /// grid.set_object_location(obj, &loc);
    /// grid.update();
    /// let mut obj_clone = obj.clone();
    /// // You should be able to fetch the original copy of the agent stored in the state through a local clone
    /// assert_eq!(grid.get_object(&obj_clone), Some(&obj));
    /// ```    
    pub fn get_object(&self, agent: &A) -> Option<&A> {
        match self.locs.get_key_value(agent) {
            Some((updated_agent, _pos)) => Some(updated_agent),
            None => None,
        }
    }

    /// Fetches the position of the agent in the grid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_ab::engine::agent::Agent;
    /// use rust_ab::engine::field::object_grid_2d::Grid2D;
    /// use rust_ab::engine::location::Int2D;
    /// use rust_ab::engine::state::State;
    ///
    /// #[derive(Copy, Clone, Eq, Hash, Debug)]
    /// struct MyObject {
    ///     pub id: i32,
    ///     pub loc: Int2D,
    /// }
    ///
    /// impl PartialEq for MyObject {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.id == other.id
    ///     }
    /// }
    ///
    /// let mut grid = Grid2D::new(10,10);
    /// let loc = Int2D{x: 2, y: 2};
    /// let mut obj = MyObject{id: 1, loc};
    /// grid.set_object_location(obj, &loc);
    /// grid.update();
    /// assert_eq!(grid.get_object_location(obj), Some(&loc));
    /// ```
    pub fn get_object_location(&self, agent: A) -> Option<&Int2D> {
        self.locs.get(&agent)
    }

    /// Fetches all the agents in a given position.
    /// If the position has never been written in, or if it has but the cell is currently empty,
    /// None is returned. Otherwise, a vec of agents in that position is returned.
    pub fn get_object_at_location(&self, pos: &Int2D) -> Option<&Vec<A>> {
        match self.locs_inversed.get(pos) {
            Some(vec) => {
                if vec.is_empty() {
                    None
                } else {
                    Some(vec)
                }
            }
            None => None,
        }
    }

    /// Updates the grid by applying the write buffer on the read one.
    pub fn update(&mut self) {
        self.locs.update();
        self.locs_inversed.update();
    }

    pub fn lazy_update(&mut self){
        self.locs.lazy_update();
        self.locs_inversed.lazy_update();
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::field::object_grid_2d::Grid2D;
    use crate::engine::location::Int2D;

    #[derive(Copy, Clone, Eq, Hash, Debug)]
    struct MyObject {
        pub id: i32,
        pub loc: Int2D,
    }

    impl PartialEq for MyObject {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    /// Test all the operations implemented on Grid2D.
    #[test]
    fn grid_2d_ops() {
        let mut grid = Grid2D::<MyObject>::new(10, 10);
        let pos = Int2D { x: 2, y: 3 };
        let pos_second = Int2D { x: 4, y: 5 };
        let agent = MyObject { id: 1, loc: pos };
        let agent_second = MyObject {
            id: 2,
            loc: pos_second,
        };

        grid.set_object_location(agent, &agent.loc);
        grid.set_object_location(agent_second, &agent_second.loc);
        grid.update();

        let agent_loc = *grid.get_object_location(agent).unwrap();
        assert_eq!(agent_loc, pos);

        let grid_agents = &*grid.get_object_at_location(&pos).unwrap();
        assert_eq!(agent, *grid_agents.first().unwrap());

        let grid_agent = grid.get_object(&agent);
        let agent_clone = agent.clone();
        assert_eq!(grid_agent, Some(&agent_clone));

        grid.remove_object(&agent);
        // This should still be set because we haven't updated the grid yet
        let agent_loc = *grid.get_object_location(agent).unwrap();
        assert_eq!(agent_loc, pos);

        grid.update();

        // Even though the agent has already been removed, this method shouldn't panic: nothing should be done
        grid.remove_object(&agent);

        let agent_loc = grid.get_object_location(agent);
        assert_eq!(agent_loc, None);
    }
}
