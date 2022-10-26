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

        /// Field with double buffering for dense matrix.
        /// You can insert/update values preserving a common state to read from in a step.
        /// As a values matrix, can contain one value per cell.
        ///
        ///
        /// A simpler version of the DenseGrid2D to use with simple values.
        /// This is useful to represent simulation spaces covered by a simple entity that can be represented with a non-agent structure.
        pub struct DenseNumberGrid2D<T: Copy + Clone + PartialEq> {

            // /// Matrix to write data. It is managed as a single Vector to improve performance.
            // pub locs: RefCell<Vec<Option<T>>>,
            // /// Matrix to read data.
            // pub rlocs: RefCell<Vec<Option<T>>>,

            pub locs: Vec<RefCell<Vec<Option<T>>>>,
            read: usize,
            write: usize,
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
                    locs: vec![
                        RefCell::new(vec![None; (width * height) as usize]),
                        RefCell::new(vec![None; (width * height) as usize])
                    ],
                    // rlocs: RefCell::new(vec![None; (width * height) as usize]),
                    read: 0,
                    write: 1,
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
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                        for value in rlocs.iter_mut() {
                            if value.is_none() {continue};
                            let result = closure(&value.unwrap());
                            *value = Some(result);
                        }

                    },
                    GridOption::WRITE => {
                        let mut locs = self.locs[self.write].borrow_mut();
                        let rlocs = self.locs[self.read].borrow();
                        for (i, value) in rlocs.iter().enumerate() {
                            if value.is_none() {continue};
                            let result = closure(&value.unwrap());
                            locs[i] = Some(result);
                        }
                    },
                    //works only with 1 element for bag
                    GridOption::READWRITE =>{

                        let mut locs = self.locs[self.write].borrow_mut();
                        let rlocs = self.locs[self.read].borrow_mut();
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

            /// Return the position of the first element that matches the given value.
            /// Return None if no element matches.
            ///
            /// # Arguments
            /// * `value` - value to search for
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                let locs = self.locs[self.read].borrow();
                for i in  0..self.width{
                    for j in 0..self.height{
                        let elem = locs[(i *  self.height + j) as usize];
                        if elem.is_some() && elem.unwrap() == value {
                            return Some(Int2D {x: i, y: j});
                        }
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
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                for i in  0..self.width{
                    for j in 0..self.height{
                        let elem = locs[(i *  self.height + j) as usize];
                        if elem.is_some() && elem.unwrap() == value {
                            return Some(Int2D {x: i, y: j});
                        }
                    }
                }
                None
            }

            /// Return all the empty bags of the read state.
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                        let index = ((i * self.height) +j) as usize;
                        if self.locs[self.read].borrow()[index].is_none() {
                            empty_bags.push(Int2D{x: i, y: j});
                        }
                    }
                }
                empty_bags
            }

            /// Return a random empty bag in rlocs. `None` if no bags are available.
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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

                let empty_bags = self.get_empty_bags();
                if empty_bags.is_empty() {
                    return None;
                }

                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..empty_bags.len());
                Some(empty_bags[index])
            }

            /// Return the value in a specific position. `None` if position is empty.
            ///
            /// # Arguments
            /// * `loc` - location to get the value from
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                let index = ((loc.x * self.height) + loc.y) as usize;
                let rlocs = self.locs[self.read].borrow();
                rlocs[index]
            }

            /// Return value of a specific position from write state. `None` if position is empty.
            ///
            /// Useful when you want to get some value written in the current step.
            /// For example, you want to get the value of a cell that is being written with a `set_value_location()`.
            ///
            /// # Arguments
            /// * `loc` - location to get the value from
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                let index = ((loc.x * self.height) + loc.y) as usize;
                let locs = self.locs[self.write].borrow();
                locs[index]
            }


            /// Read and call a closure to all values inside Read state
            ///
            /// # Arguments
            /// * `closure` - closure to apply to all values
            ///
            /// # Example
            /// ```
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                        &T, //value
                    )
            {
                for i in 0 .. self.width{
                    for j in 0 .. self.height{
                        let index = ((i * self.height) + j) as usize;
                        let locs = self.locs[self.read].borrow()[index];
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
            ///
            /// # Example
            ///
            /// ```rust
            /// let mut grid = DenseNumberGrid2D::<u16>::new(10, 10);
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
                    &T //value
                )
            {
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let index = ((i * self.height) + j) as usize;
                        let locs = self.locs[self.write].borrow()[index];
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
            /// Thanks to the double buffering, you havent to worry about remove value of previous step.
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
                let index = ((loc.x * self.height) + loc.y) as usize;
                let mut locs = self.locs[self.write].borrow_mut();
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
            ///
            pub fn remove_value_location(&self, loc: &Int2D) {
                let mut locs = self.locs[self.write].borrow_mut();
                let index = ((loc.x * self.height) + loc.y) as usize;
                locs[index] = None;
            }


        }

        impl<T: Copy + Clone + PartialEq> Field for DenseNumberGrid2D<T> {
            /// Swap read and write states of the field and clear write State
            fn lazy_update(&mut self){

                std::mem::swap(&mut self.read, &mut self.write);

                let mut locs = self.locs[self.write].borrow_mut();
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
                let mut locs = self.locs[self.write].borrow_mut();
                let mut rlocs = self.locs[self.read].borrow_mut();
                for i in 0..locs.len(){
                    rlocs[i] = locs[i];
                    locs[i] = None;
                }
            }
        }

    }
}
