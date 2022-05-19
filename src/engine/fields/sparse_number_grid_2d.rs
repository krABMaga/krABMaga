use crate::engine::{
    fields::{field::Field, grid_option::GridOption},
    location::Int2D,
};

use crate::rand::Rng;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use std::cell::RefCell;
        use hashbrown::HashMap;
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
            pub locs: RefCell<HashMap<Int2D, T>>,
            /// Hashmap to read data. Key is the value, value is the location.
            pub rlocs: RefCell<HashMap<Int2D, T>>,
            /// First dimension of the field
            pub width: i32,
            /// Second dimension of the field
            pub height: i32
        }

        impl<T: Copy + Clone + PartialEq> SparseNumberGrid2D<T> {
            /// create a new instance of SparseNumberenseGrid2D
            ///
            /// # Arguments
            /// * `width` - first dimension of the field
            /// * `height` - second dimension of the field
            pub fn new(width: i32, height: i32) -> SparseNumberGrid2D<T> {
                SparseNumberGrid2D {
                    locs: RefCell::new(HashMap::new()),
                    rlocs: RefCell::new(HashMap::new()),
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
            pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
            where
                F: Fn(&T) -> T,
            {

                match option {
                    GridOption::READ => {
                        let mut rlocs = self.rlocs.borrow_mut();
                        for (_key, value) in rlocs.iter_mut() {
                            *value = closure(value);
                        }
                    },
                    GridOption::WRITE => {
                        let mut locs = self.locs.borrow_mut();
                        for (_key, value) in locs.iter_mut() {
                            *value = closure(value);
                        }
                    },
                    //update the write state using the write values if exist, otherwise use the read values
                    GridOption::READWRITE => {
                        let rlocs = self.rlocs.borrow();
                        let mut locs = self.locs.borrow_mut();

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
            pub fn iter_values<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &T//value
                )
            {
                let rlocs = self.rlocs.borrow();
                for (key, val) in rlocs.iter(){
                    closure(key, val);
                }
            }

            /// Iterate over all valuse inside the field and apply the closure.
            /// Useful when you want to access to all the objects changed/executed into the current step.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to each element of the matrix
            pub fn iter_values_unbuffered<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &T, //value
                )
            {
                let locs = self.locs.borrow();
                for (key, val) in locs.iter(){
                    closure(key, val);
                }
            }


            /// Get all empty bags from read state.
            pub fn get_empty_bags(&self) -> Vec<Int2D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let loc = Int2D{x: i, y: j};
                        match self.rlocs.borrow().get(&loc){
                            Some(_bag) =>{},
                            None => {
                                empty_bags.push(Int2D{x: i, y: j});
                            }
                        }
                    }
                }
                empty_bags
            }

            /// Get one random empty bag from read state. `None` if no empty bag is found.
            pub fn get_random_empty_bag(&self) -> Option<Int2D>{
                let mut rng = rand::thread_rng();
                loop {
                    let loc = Int2D{x: rng.gen_range(0..self.width), y: rng.gen_range(0..self.height)};
                    match self.rlocs.borrow().get(&loc){
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
            pub fn get_location(&self, value: T) -> Option<Int2D> {
                let rlocs = self.rlocs.borrow();
                for (key, val) in rlocs.iter() {
                    if *val == value {
                        return Some(*key);
                    }
                }
                None
            }

            /// Return the position of the first element that matches the given value.
            /// Return None if no element matches.
            /// It will return the value from the write state.
            ///
            /// # Arguments
            /// * `value` - value to search for
            pub fn get_location_unbuffered(&self, value: T) -> Option<Int2D> {
                let locs = self.locs.borrow();
                for (key, val) in locs.iter() {
                    if *val == value {
                        return Some(*key);
                    }
                }
                None
            }


            /// Get the value at a specific location.
            ///
            /// # Arguments
            /// * `loc` - location to get the value from
            pub fn get_value(&self, loc: &Int2D) -> Option<T> {
                let rlocs = self.rlocs.borrow();
                rlocs.get(loc).copied()
            }

            /// Return value of a specific position from write state. `None` if position is empty.
            ///
            /// Useful when you want to get some value written in the current step.
            /// For example, you want to get the value of a cell that is being written with a `set_value_location()`
            ///
            /// # Arguments
            /// * `loc` - location to get the value from
            pub fn get_value_unbuffered(&self, loc: &Int2D) -> Option<T> {
                let locs = self.locs.borrow();
                locs.get(loc).copied()
            }

            /// Insert a value in a specific position.
            /// Double buffering swap the write and read state at the end of the step, so you have to call this function also if the value is not changed.
            ///
            /// If the position is empty, the value is pushed in the bag.
            /// If the position is not empty, the value is pushed in the bag and the old value is dropped.
            ///
            /// # Arguments
            /// * `value` - value to set at the location
            /// * `loc` - location to set the value at
            pub fn set_value_location(&self, value: T, loc: &Int2D) {
                let mut locs = self.locs.borrow_mut();
                locs.insert(*loc, value);
            }

            /// Remove a value from write state.
            /// You have to use it to remove a value written/updated in this step.
            /// Double buffering swap the write and read state at the end of the step, so you have to call
            /// this function only if the value was written/set in this step.
            ///
            /// # Arguments
            /// * `loc` - location to remove the value
            pub fn remove_value_location(&self, loc: &Int2D) {
                let mut locs = self.locs.borrow_mut();
                locs.remove(loc);
            }

        }

        impl<T: Copy + Clone + PartialEq> Field for SparseNumberGrid2D<T> {
            /// Swap the state of the field and clear locs
            fn lazy_update(&mut self) {
                unsafe {
                    std::ptr::swap(
                        self.rlocs.as_ptr(),
                        self.locs.as_ptr(),
                    )
                }
                self.locs.borrow_mut().clear();
            }

            /// Swap the state of the field and updates the rlocs matrix
            fn update(&mut self) {
                let mut rlocs = self.rlocs.borrow_mut();
                for (key, value) in self.locs.borrow().iter() {
                    rlocs.insert(*key, *value);
                }
            }
        }
    }
}
