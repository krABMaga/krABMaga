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

        /// Field with double buffering for sparse matrix
        pub struct SparseNumberGrid2D<T: Copy + Clone> {
            /// Hashmap to write data. Key is location, value is the number.
            pub locs: RefCell<HashMap<Int2D, T>>,
            /// Hashmap to read data. Key is the value, value is the location.
            pub rlocs: RefCell<HashMap<Int2D, T>>,
            /// First dimension of the field
            pub width: i32,
            /// Second dimension of the field
            pub height: i32
        }

        impl<T: Copy + Clone> SparseNumberGrid2D<T> {
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
            /// * `value` - object to remove
            /// * `loc` - location to remove the object
            pub fn remove_value_location(&self, _value: T, loc: &Int2D) {
                let mut locs = self.locs.borrow_mut();
                locs.remove(loc);
            }

        }

        impl<T: Copy + Clone> Field for SparseNumberGrid2D<T> {
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
