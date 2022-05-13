use crate::engine::{
    fields::{field::Field, grid_option::GridOption},
    location::Int2D,
};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use crate::rand::Rng;
        use std::cell::RefCell;
    }
}

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        pub struct DenseNumberGrid2D<T: Copy + Clone> {
            pub locs: DBDashMap<Int2D, T>,
            pub width: i32,
            pub height: i32,
        }

        impl<T: Copy + Clone > DenseNumberGrid2D<T> {
            pub fn new(width: i32, height: i32) -> DenseNumberGrid2D<T> {
                DenseNumberGrid2D {
                    locs: DBDashMap::new(),
                    width: width.abs(),
                    height: height.abs(),
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

        impl<T: Copy + Clone> Field for DenseNumberGrid2D<T> {
            fn lazy_update(&mut self) {
                self.locs.lazy_update();
            }

            fn update(&mut self) {
                self.locs.update();
            }
        }

    } else {

        /// Matrix with double buffering.
        /// You can insert/update values preserving a common state to read from in a step.
        ///
        ///
        /// A simpler version of the DenseGrid2D to use with simple values.
        /// This is useful to represent simulation spaces covered by a simple entity that can be represented with a non-agent structure.
        pub struct DenseNumberGrid2D<T: Copy + Clone + PartialEq> {

            /// Matrix to write data. It is managed as a single Vector to improve performance.
            pub locs: RefCell<Vec<Option<T>>>,
            /// Matrix to read data.
            pub rlocs: RefCell<Vec<Option<T>>>,
            /// First dimension of the field
            pub width: i32,
            /// Second dimension of the field
            pub height: i32
        }

        impl<T: Copy + Clone + PartialEq> DenseNumberGrid2D<T> {
            /// Create new instance of DenseNumberGrid2D
            ///
            /// # Arguments
            /// * `width` - First dimension of the field
            /// * `height` - Second dimension of the field
            pub fn new(width: i32, height: i32) -> DenseNumberGrid2D<T> {
                DenseNumberGrid2D {
                    // locs: RefCell::new(std::iter::repeat_with(Vec::new).take((width * height) as usize).collect()),
                    // rlocs: RefCell::new(std::iter::repeat_with(Vec::new).take((width * height)as usize).collect()),
                    locs: RefCell::new(vec![None; (width * height) as usize]),
                    rlocs: RefCell::new(vec![None; (width * height) as usize]),
                    width: width.abs(),
                    height: height.abs(),
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
                        // let mut rlocs = self.rlocs.borrow_mut();
                        // for i in 0 .. rlocs.len() {
                        //     let mut vec = Vec::new();
                        //     if rlocs[i].is_none() {continue};
                        //     let result = closure(elem);
                        //     rlocs[i] = vec;
                        // }
                        let mut rlocs = self.rlocs.borrow_mut();
                        for value in rlocs.iter_mut() {
                            if value.is_none() {continue};
                            let result = closure(&value.unwrap());
                            *value = Some(result);
                        }

                    },
                    GridOption::WRITE => {
                        // let mut locs = self.locs.borrow_mut();
                        // let rlocs = self.rlocs.borrow();
                        // for i in 0 .. rlocs.len() {
                        //     if rlocs[i].is_empty() {continue};
                        //     for elem in rlocs[i].iter() {
                        //         let result = closure(elem);
                        //         locs[i].push(result);
                        //     }
                        // }

                        let mut locs = self.locs.borrow_mut();
                        let rlocs = self.rlocs.borrow();
                        for (i, value) in rlocs.iter().enumerate() {
                            if value.is_none() {continue};
                            let result = closure(&value.unwrap());
                            locs[i] = Some(result);
                        }
                    },
                    //works only with 1 element for bag
                    GridOption::READWRITE =>{
                        // let mut locs = self.locs.borrow_mut();
                        // let rlocs = self.rlocs.borrow_mut();
                        // // for each bag in read
                        // for i in 0..rlocs.len() {
                        //     // calculate the bag_id
                        //     // if the corresponding write bag is not empty
                        //     if !locs[i].is_empty() {
                        //         // for each element in the write bag
                        //         for elem in locs[i].iter_mut() {
                        //             // apply the closure
                        //             let result = closure(elem);
                        //             *elem = result;
                        //         }
                        //     }else{ // else if the corresponding bag is not empty
                        //         // if the read bag is empty go to the next iteration
                        //         if rlocs[i].is_empty() { continue }
                        //         // for each element in the read bag
                        //         for elem in rlocs[i].iter() {
                        //             // apply the closure
                        //             let result = closure(elem);
                        //             if !locs[i].contains(&result){
                        //                 // push it
                        //                 locs[i].push(result);
                        //             }
                        //         }
                        //     }
                        // }

                        let mut locs = self.locs.borrow_mut();
                        let rlocs = self.rlocs.borrow_mut();
                        for (i, elem) in rlocs.iter().enumerate() {
                            if let Some(value) = locs[i] {
                                let result = closure(&value);
                                locs[i] = Some(result);
                            }
                            else if let Some(value) = elem {
                                let result = closure(value);
                                locs[i] = Some(result);
                            }
                        }
                    }
                }
            }

            /// Return all the empty bags in rlocs
            pub fn get_empty_bags(&self) -> Vec<Int2D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let index = ((i * self.height) +j) as usize;
                        if self.rlocs.borrow()[index].is_none() {
                            empty_bags.push(Int2D{x: i, y: j});
                        }
                    }
                }
                empty_bags
            }

            /// Return a random empty bag in rlocs. `None` if no bags are available
            pub fn get_random_empty_bag(&self) -> Option<Int2D>{

                let empty_bags = self.get_empty_bags();
                if empty_bags.is_empty() {
                    return None;
                }

                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..empty_bags.len());
                Some(empty_bags[index])
            }

            /// Return the first value of a specific position. `None` if position is empty.
            ///
            /// # Arguments
            /// * `loc` - position to get the value
            pub fn get_value(&self, loc: &Int2D) -> Option<T> {
                let index = ((loc.x * self.height) + loc.y) as usize;
                let rlocs = self.rlocs.borrow();
                rlocs[index]
            }

            /// Return all values of a specific position from write state. `None` if position is empty.
            ///
            /// Useful when you want to get some value written in the current step.
            /// For example, you want to get the value of a cell that is being written with a `set_value_location()`.
            ///
            /// # Arguments
            /// * `loc` - position to get the values
            pub fn get_value_unbuffered(&self, loc: &Int2D) -> Option<T> {
                let index = ((loc.x * self.height) + loc.y) as usize;
                let locs = self.locs.borrow();
                locs[index]
            }


            /// Read and apply a closure to all values inside Read state
            ///
            /// # Arguments
            /// * `closure` - closure to apply to all values
            pub fn iter_values<F>(&self, closure: F)
                where
                    F: Fn(
                        &Int2D, //location
                        &T, //value
                    )
            {
                for i in 0 .. self.width{
                    for j in 0 .. self.height{
                        let index = ((i * self.height) + j) as usize;
                        let locs = self.rlocs.borrow()[index];
                        if let Some(value) = locs {
                            closure(&Int2D{x: i, y: j}, &value);
                        }
                        // if !locs.is_empty() {
                        //     for obj in locs{
                        //         closure(&Int2D{x: i, y: j}, obj);
                        //     }
                        // }
                    }
                }
            }

            /// Read and apply a closure to all values inside Write state.
            /// Useful when you want to iterate over all values written in the current step.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to all values
            pub fn iter_values_unbuffered<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &T //value
                )
            {
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let index = ((i * self.height) + j) as usize;
                        let locs = self.locs.borrow()[index];
                        if let Some(value) = locs {
                            closure(&Int2D{x: i, y: j}, &value);
                        }
                        // if !locs.is_empty() {
                        //     for obj in locs{
                        //         closure(&loc, obj);
                        //     }
                        // }
                    }
                }
            }


            /// Write a value in a specific position.
            /// Thanks to the double buffering, you havent to worry about remove value from position of previous step.
            ///
            /// If the position is empty, the value is pushed in the bag.
            /// If the position is not empty, the value is pushed in the bag and the old value is dropped.
            ///
            /// # Arguments
            /// * `value` - value to write
            /// * `loc` - position to write the value
            pub fn set_value_location(&self, value: T, loc: &Int2D) {
                let index = ((loc.x * self.height) + loc.y) as usize;
                let mut locs = self.locs.borrow_mut();
                locs[index] = Some(value);

                // if !locs[index].is_empty() {
                //     locs[index].retain(|&obj| obj != value);
                // }
                // locs[index].push(value);
            }

            /// Remove a value from write state.
            /// You have to use it to remove a value written/updated in this step.
            /// Double buffering swap the write and read state at the end of the step, so you have to call
            /// this function only if the value was written/set in this step.
            ///
            /// # Arguments
            /// * `loc` - location to remove the object
            pub fn remove_value_location(&self, loc: &Int2D) {
                let mut locs = self.locs.borrow_mut();
                let index = ((loc.x * self.height) + loc.y) as usize;
                locs[index] = None;
            }


        }

        impl<T: Copy + Clone + PartialEq> Field for DenseNumberGrid2D<T> {
            /// Swap read and write states of the field and clear write State
            fn lazy_update(&mut self){
                unsafe {
                    std::ptr::swap(
                        self.locs.as_ptr(),
                        self.rlocs.as_ptr(),
                    )
                }
                let mut locs = self.locs.borrow_mut();
                //set None all elements
                for i in 0..locs.len(){
                    locs[i] = None;
                }

                // for i in 0..locs.len(){
                //     locs[i].clear();
                // }
            }

            /// Copy values from write state into read one
            fn update(&mut self) {
                // copy locs to rlocs
                let mut locs = self.locs.borrow_mut();
                let mut rlocs = self.rlocs.borrow_mut();
                for i in 0..locs.len(){
                    rlocs[i] = locs[i];
                    locs[i] = None;
                }
            }
        }

    }
}
