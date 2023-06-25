use crate::engine::{
    fields::{field::Field, grid_option::GridOption},
    location::Int2D,
};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use std::cell::RefCell;
        use hashbrown::HashMap;
        use crate::rand::Rng;
    }
}

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        pub struct SparseNumberGrid2D<T: Copy + Clone> {
            pub locs: DBDashMap<Int2D, T>,
            pub width: i32,
            pub height: i32,
        }

        impl<T: Copy + Clone> SparseNumberGrid2D<T> {
            pub fn new(width: i32, height: i32) -> SparseNumberGrid2D<T> {
                SparseNumberGrid2D {
                    locs: DBDashMap::new(),
                    width,
                    height,
                }
            }

            pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
            where
                F: Fn(&T) -> T,
            {
                match option {
                    GridOption::READ => {
                        self.locs.apply_to_all_values(closure);
                    },
                    GridOption::WRITE => {
                        self.locs.apply_to_all_values_write(closure);
                    },
                    GridOption::READWRITE => {
                        self.locs.apply_to_all_values_read_write(closure);
                    }
                }
            }

            pub fn get_value(&self, loc: &Int2D) -> Option<T> {
                match self.locs.get_read(loc){
                    Some(value) => Some(*value),
                    None => None
                }
            }

            pub fn get_value_unbuffered(&self, loc: &Int2D) -> Option<T> {
                match self.locs.get_write(loc){
                    Some(value) => Some(*value),
                    None => None
                }
            }

            pub fn set_value_location(&self, value: T, loc: &Int2D) {
                self.locs.insert(*loc, value);
            }
        }

        impl<T: Copy + Clone> Field for SparseNumberGrid2D<T> {
            fn lazy_update(&mut self) {
                self.locs.lazy_update();
            }

            fn update(&mut self) {
                self.locs.update();
            }
        }

    } else {

        /// Field with double buffering for sparse matrix.
        /// You can insert/update values preserving a common state to read from in a step.
        /// As a values matrix, can contain one value per cell.
        ///
        /// A simpler version of the SparseGrid2D to use with simple values.
        /// This is useful to represent simulation spaces covered by a simple entity that can be represented with a non-agent structure.
        pub struct SparseNumberGrid2D<T: Copy + Clone + PartialEq> {
            /// Hashmap to write data. Key is location, value is the number.
            // pub locs: RefCell<HashMap<Int2D, T>>,
            /// Hashmap to read data. Key is the value, value is the location.
            // pub rlocs: RefCell<HashMap<Int2D, T>>,
            pub locs: Vec<RefCell<HashMap<Int2D, T>>>,
            read: usize,
            write: usize,
            /// First dimension of the field
            pub width: i32,
            /// Second dimension of the field
            pub height: i32
        }

        impl<T: Copy + Clone + PartialEq> SparseNumberGrid2D<T> {
            /// create a new instance of SparseNumberGrid2D
            ///
            /// # Arguments
            /// * `width` - first dimension of the field
            /// * `height` - second dimension of the field
            pub fn new(width: i32, height: i32) -> SparseNumberGrid2D<T> {
                SparseNumberGrid2D {
                    // locs: RefCell::new(HashMap::new()),
                    // rlocs: RefCell::new(HashMap::new()),
                    locs: vec![RefCell::new(HashMap::new()), RefCell::new(HashMap::new())],
                    read: 0,
                    write: 1,
                    width,
                    height,
                }
            }

            /// Apply a closure to all values.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to all values
            /// * `option` - option to read or write
            /// ## `option` possible values
            /// * `READ` - update the values from rlocs
            /// * `WRITE` - update the values from locs
            /// * `READWRITE` - check locs and rlocs simultaneously to apply the closure
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// for i in 0..10 {
            ///    for j in 0..10 {
            ///       grid.set_value_location(i as u16, &Int2D::new(i, j));
            ///   }
            /// }
            ///
            /// // Need WRITE or READWRITE option to update the values
            /// // because Read state isn't updated
            /// grid.apply_to_all_values(|x| x + 1, GridOption::WRITE);
            ///
            /// grid.lazy_update();
            /// grid.apply_to_all_values(|x| x - 1, GridOption::READ);
            ///
            /// ```
            pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
            where
                F: Fn(&T) -> T,
            {

                match option {
                    GridOption::READ => {
                        let mut rlocs = self.locs[self.read].borrow_mut();
                        for (_key, value) in rlocs.iter_mut() {
                            *value = closure(value);
                        }
                    },
                    GridOption::WRITE => {
                        let mut locs = self.locs[self.write].borrow_mut();
                        for (_key, value) in locs.iter_mut() {
                            *value = closure(value);
                        }
                    },
                    //update the write state using the write values if exist, otherwise use the read values
                    GridOption::READWRITE => {
                        let rlocs = self.locs[self.read].borrow();
                        let mut locs = self.locs[self.write].borrow_mut();

                        // for each bag in read
                        for (key, value) in rlocs.iter() {
                            if let Some(write_value) = locs.get_mut(key){
                                *write_value = closure(write_value);
                            }else{
                                locs.insert(*key, closure(value));
                            }
                        }

                    }
                }
            }


            /// Read and call a closure to all values inside Read state
            /// # Arguments
            /// * `closure` - closure to apply to all values
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// for i in 0..10 {
            ///     for j in 0..10 {
            ///         grid.set_value_location(1, &Int2D::new(i, j));
            ///     }
            /// }
            ///
            /// grid.lazy_update();
            /// grid.iter_values(|&loc, &value| {
            ///     // do something with loc and value
            ///     // can't modify the grid here
            /// };
            ///
            /// ```
            pub fn iter_values<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &T//value
                )
            {
                let rlocs = self.locs[self.read].borrow();
                for (key, val) in rlocs.iter(){
                    closure(key, val);
                }
            }

            /// Iterate over all values inside the field and apply the closure.
            /// Useful when you want to access to all the objects changed/executed into the current step.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to each element of the matrix
            ///
            /// # Example
            ///
            /// ```rust
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// for i in 0..10 {
            ///     for j in 0..10 {
            ///       grid.set_value_location(1, &Int2D::new(i, j));
            ///     }
            /// }
            ///
            /// // can't modify the grid here
            /// grid.iter_values_unbuffered(|&loc, &value| {
            ///    some_function(loc, value);
            /// };
            ///
            /// ```
            pub fn iter_values_unbuffered<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &T, //value
                )
            {
                let locs = self.locs[self.write].borrow();
                for (key, val) in locs.iter(){
                    closure(key, val);
                }
            }


            /// Return all the empty bags of the read state.
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// let empty = grid.get_empty_bags();
            /// assert_eq!(empty.len(), 100);
            ///
            /// for i in 0..10 {
            ///   for j in 0..10 {
            ///      grid.set_value_location(1, &Int2D::new(i, j));
            ///   }
            /// }
            ///
            /// // Before an update, the grid is not updated, so the empty bags are still available
            /// let empty = grid.get_empty_bags();
            /// assert_eq!(empty.len(), 100);
            ///
            /// grid.lazy_update();
            /// let empty = grid.get_empty_bags();
            /// assert_eq!(empty.len(), 0);
            ///
            pub fn get_empty_bags(&self) -> Vec<Int2D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let loc = Int2D{x: i, y: j};
                        match self.locs[self.read].borrow().get(&loc){
                            Some(_bag) =>{},
                            None => {
                                empty_bags.push(Int2D{x: i, y: j});
                            }
                        }
                    }
                }
                empty_bags
            }

            /// Return a random empty bag in rlocs. `None` if no bags are available.
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// let empty = grid.get_random_empty_bag();
            /// assert(empty.is_some());
            ///
            /// grid.set_value_location(1, &empty.unwrap());
            /// grid.lazy_update();
            ///
            /// let empty2 = grid.get_random_empty_bag();
            /// assert(empty2.is_some());
            /// assert_ne!(empty.unwrap(), empty2.unwrap());
            ///
            /// ```
            pub fn get_random_empty_bag(&self) -> Option<Int2D>{
                let mut rng = rand::thread_rng();
                loop {
                    let loc = Int2D{x: rng.gen_range(0..self.width), y: rng.gen_range(0..self.height)};
                    match self.locs[self.read].borrow().get(&loc){
                        Some(_bag) =>{},
                        None => {
                            return Some(loc)
                        }
                    }
                }
            }

            /// Return the position of the first element that matches the given value.
            /// Return None if no element matches.
            ///
            /// # Arguments
            /// * `value` - value to search for
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// grid.set_value_location(1, &Int2D::new(5, 5));
            /// grid.set_value_location(1, &Int2D::new(6, 6));
            ///
            /// grid.lazy_update();
            /// let pos = grid.get_location(1);
            /// assert_eq!(pos, Some(Int2D::new(5, 5)));
            ///
            /// let none = grid.get_location(2);
            /// assert_eq!(none, None);
            /// ```
            ///
            pub fn get_location(&self, value: T) -> Option<Int2D> {
                let rlocs = self.locs[self.read].borrow();
                for (key, val) in rlocs.iter() {
                    if *val == value {
                        return Some(*key);
                    }
                }
                None
            }

            /// Return the position of the first element that matches the given value from write state.
            /// Return None if no element matches.
            ///
            /// # Arguments
            /// * `value` - value to search for
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// grid.set_value_location(1, &Int2D::new(5, 5));
            /// grid.set_value_location(1, &Int2D::new(6, 6));
            ///
            /// // Work on write state, so on unupdated state
            /// let pos = grid.get_location_unbuffered(1);
            /// assert_eq!(pos, Some(Int2D::new(5, 5)));
            ///
            /// let none = grid.get_location_unbuffered(2);
            /// assert_eq!(none, None);
            ///
            /// grid.lazy_update();
            /// let pos = grid.get_location_unbuffered(1);
            /// assert_eq!(pos, None);
            /// ```
            ///
            pub fn get_location_unbuffered(&self, value: T) -> Option<Int2D> {
                let locs = self.locs[self.write].borrow();
                for (key, val) in locs.iter() {
                    if *val == value {
                        return Some(*key);
                    }
                }
                None
            }


            /// Return the value in a specific position. `None` if position is empty.
            ///
            /// # Arguments
            /// * `loc` - location to get the value from
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// grid.set_value_location(1, &Int2D::new(5, 5));
            ///
            /// let value = grid.get_value(&Int2D::new(5, 5));
            /// assert_eq!(value, None);
            ///
            /// grid.lazy_update();
            ///
            /// let value = grid.get_value(&Int2D::new(5, 5));
            /// assert_eq!(value, Some(1));
            /// ```
            ///
            pub fn get_value(&self, loc: &Int2D) -> Option<T> {
                let rlocs = self.locs[self.read].borrow();
                rlocs.get(loc).copied()
            }

            /// Return value of a specific position from write state. `None` if position is empty.
            ///
            /// Useful when you want to get some value written in the current step.
            /// For example, you want to get the value of a cell that is being written with a `set_value_location()`
            ///
            /// # Arguments
            /// * `loc` - location to get the value from
            ///
            /// # Example
            /// ```
            /// let mut grid = SparseNumberGrid2D::<u16>::new(10, 10);
            /// grid.set_value_location(1, &Int2D::new(5, 5));
            /// let value = grid.get_value_unbuffered(&Int2D::new(5, 5));
            /// assert_eq!(value, Some(1));
            ///
            /// grid.lazy_update();
            /// let value = grid.get_value_unbuffered(&Int2D::new(5, 5));
            /// assert_eq!(value, None);
            ///
            /// ```
            pub fn get_value_unbuffered(&self, loc: &Int2D) -> Option<T> {
                let locs = self.locs[self.write].borrow();
                locs.get(loc).copied()
            }

            /// Write a value in a specific position.
            /// Double buffering swap the write and read state at the end of the step, so you have to call this function also if the value is not changed.
            ///
            /// # Arguments
            /// * `value` - value to set at the location
            /// * `loc` - location to set the value at
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
            /// grid.set_value_location(1, &Int2D::new(5, 5));
            /// grid.set_value_location(2, &Int2D::new(5, 5));
            ///
            /// grid.lazy_update();
            ///
            /// let value = grid.get_value(&Int2D::new(5, 5));
            /// assert_ne!(value, Some(1));
            /// assert_eq!(value, Some(2));
            /// ```
            pub fn set_value_location(&self, value: T, loc: &Int2D) {
                let mut locs = self.locs[self.write].borrow_mut();
                locs.insert(*loc, value);
            }

            /// Remove a value from write state.
            /// You have to use it to remove a value written/updated in this step.
            /// Double buffering swap the write and read state at the end of the step, so you have to call
            /// this function only if the value was written/set in this step.
            ///
            /// # Arguments
            /// * `loc` - location to remove the value from
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
            /// grid.set_value_location(1, &Int2D::new(5, 5));
            /// grid.remove_value_location(&Int2D::new(5, 5));
            ///
            /// let value = grid.get_value_unbuffered(&Int2D::new(5, 5));
            /// assert_eq!(value, None);
            ///
            /// grid.lazy_update();
            /// let value = grid.get_value(&Int2D::new(5, 5));
            /// assert_eq!(value, None);
            ///
            /// ```
            pub fn remove_value_location(&self, loc: &Int2D) {
                let mut locs = self.locs[self.write].borrow_mut();
                locs.remove(loc);
            }

        }

        impl<T: Copy + Clone + PartialEq> Field for SparseNumberGrid2D<T> {
            /// Swap the state of the field and clear locs
            fn lazy_update(&mut self) {
                std::mem::swap(&mut self.read, &mut self.write);
                self.locs[self.write].borrow_mut().clear();
            }

            /// Swap the state of the field and updates the rlocs matrix
            fn update(&mut self) {
                let mut rlocs = self.locs[self.read].borrow_mut();
                rlocs.clear();
                for (key, value) in self.locs[self.write].borrow().iter() {
                    rlocs.insert(*key, *value);
                }
                self.locs[self.write].borrow_mut().clear();
            }
        }
    }
}
