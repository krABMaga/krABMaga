use crate::engine::{
    fields::{field::Field, grid_option::GridOption},
    location::Int2D,
};

use cfg_if::cfg_if;
use std::hash::Hash;

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use std::cell::RefCell;
        use crate::rand::Rng;
    }
}

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        pub struct DenseGrid2D<O: Eq + Hash + Clone + Copy> {
            pub obj2loc: DBDashMap<O, Int2D>, // old locs
            pub loc2objs: DBDashMap<Int2D, Vec<O>>, // old locs_inversed
            pub width: i32,
            pub height: i32,
        }

        impl<O: Eq + Hash + Clone + Copy> DenseGrid2D<O> {
            pub fn new(width: i32, height: i32) -> DenseGrid2D<O> {
                DenseGrid2D {
                    obj2loc: DBDashMap::with_capacity((width * height) as usize),
                    loc2objs: DBDashMap::with_capacity((width * height) as usize),
                    width: width.abs(),
                    height: height.abs(),
                }
            }

            pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
            where
                F: Fn(&Int2D, &O) -> Option<O>,
            {
                match option {
                    GridOption::READ => {
                        self.obj2loc.apply_to_all_keys(closure);
                    },
                    GridOption::WRITE => {
                        self.obj2loc.apply_to_all_keys(closure);
                    },
                    GridOption::READWRITE =>{
                        self.obj2loc.apply_to_all_keys(closure);

                    }
                }
            }

            pub fn get(&self, object: &O) -> Option<O> {
                match self.obj2loc.get_key_value(object) {
                    Some((updated_object, _loc)) => Some(*updated_object),
                    None => None,
                }
            }

            pub fn get_objects(&self, loc: &Int2D) -> Option<Vec<O>> {
                match self.loc2objs.get_read(loc) {
                    Some(vec) => {
                        if vec.is_empty() {
                            None
                        } else {
                            Some(vec.to_vec())
                        }
                    }
                    None => None,
                }
            }

            pub fn get_objects_unbuffered(&self, loc: &Int2D) -> Option<Vec<O>> {
                match self.loc2objs.get_write(loc) {
                    Some(vec) => {
                        if vec.is_empty() {
                            None
                        } else {
                            Some(vec.to_vec())
                        }
                    }
                    None => None,
                }
            }

            pub fn get_location(&self, object: O) -> Option<Int2D> {
                match self.obj2loc.get_read(&object) {
                    Some(updated_object) => Some(*updated_object),
                    None => None,
                }
            }

            pub fn get_location_unbuffered(&self, object: O) -> Option<Int2D> {
                match self.obj2loc.get_write(&object) {
                    Some(updated_object) => Some(*updated_object),
                    None => None,
                }
            }

            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &O //value
                )
            {
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let loc = Int2D{x: i, y: j};
                        let bag = self.loc2objs.get_read(&loc);
                        match bag {
                            Some(bag) =>{
                                for obj in bag{
                                    closure(&loc, &obj);
                                }
                            },
                            None => {}
                        }
                    }
                }
            }

            pub fn remove_object(&self, object: &O) {
                if let Some(old_loc) = self.obj2loc.get_read(object) {
                    self.loc2objs
                        .get_write(old_loc)
                        .expect("error in remove object")
                        .value_mut()
                        .retain(|&x| x != *object);
                }
                self.obj2loc.remove(object);
            }

            pub fn remove_object_location(&self, object: O, loc: &Int2D) {
                match self.loc2objs.get_write(loc) {
                    Some(mut vec) => {
                        if !vec.is_empty() {
                            vec.retain(|&x| x != object);
                        }
                    }
                    None => { /* do nothing */ },
                }
            }

            pub fn set_object_location(&self, object: O, loc: &Int2D) {
                match self.loc2objs.get_write(loc) {
                    Some(mut vec) => {
                        if !vec.is_empty() {
                            vec.retain(|&x| x != object);
                        }
                        vec.push(object);
                    }
                    None => { self.loc2objs.insert(*loc, vec![object]);},
                }
                self.obj2loc.insert(object, *loc);
            }

        }
        impl<O: Eq + Hash + Clone + Copy> Field for DenseGrid2D<O> {

            fn lazy_update(&mut self){
                self.obj2loc.lazy_update();
                self.loc2objs.lazy_update();
            }

            fn update(&mut self) {
                self.obj2loc.update();
                self.loc2objs.update();
            }
        }


}else{
            /// Matrix with double buffering.
            ///
            /// You can insert/update objects preserving a common state to read from in a step.
            pub struct DenseGrid2D<O: Eq + Hash + Clone + Copy> {
                /// Matrix to write data. Vector of vectors that have a generic Object O inside
                /// The outer vector represents the whole field, the inner vector represents the objects inside a cell
                // pub locs: RefCell<Vec<Vec<O>>>,
                /// Matrix to read data. Vector of vectors that have a generic Object O inside
                // pub rlocs: RefCell<Vec<Vec<O>>>,
                pub locs: Vec<RefCell<Vec<Vec<O>>>>,
                read: usize,
                write: usize,
                /// First dimension of the field
                pub width: i32,
                /// Second dimension of the field
                pub height: i32,
            }

            impl<O: Eq + Hash + Clone + Copy> DenseGrid2D<O> {

                /// Create a new instance of DenseGrid2D
                ///
                /// # Arguments
                /// * `width` - first dimension of the field
                /// * `height` - second dimension of the field
                pub fn new(width: i32, height: i32) -> DenseGrid2D<O> {
                    DenseGrid2D {
                        // locs: RefCell::new(std::iter::repeat_with(Vec::new).take((width * height) as usize).collect()),
                        // rlocs: RefCell::new(std::iter::repeat_with(Vec::new).take((width * height)as usize).collect()),
                        locs: vec![
                            RefCell::new(std::iter::repeat_with(Vec::new).take((width * height) as usize).collect()),
                            RefCell::new(std::iter::repeat_with(Vec::new).take((width * height) as usize).collect()),
                        ],
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
                pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
                where
                    F: Fn(&Int2D, &O) -> Option<O>,
                {
                    match option {
                        GridOption::READ => {
                            let mut rlocs = self.locs[self.read].borrow_mut();
                            for i in 0 .. rlocs.len() {
                                let bag_id = calculate_indexes_bag(i as i32, self.width, self.height).expect("error in calculate_indexes_bag");
                                let mut vec = Vec::new();
                                if rlocs[i].is_empty() {continue};
                                for elem in rlocs[i].iter() {
                                    let result = closure(&bag_id, elem);
                                    if let Some(x) = result {
                                        vec.push(x)
                                    }
                                }
                                rlocs[i] = vec;
                            }
                        },
                        GridOption::WRITE => {
                            let mut locs = self.locs[self.write].borrow_mut();
                            let rlocs = self.locs[self.read].borrow();
                            for i in 0 .. rlocs.len() {
                                let bag_id = calculate_indexes_bag(i as i32, self.width, self.height).expect("error in calculate_indexes_bag");
                                if rlocs[i].is_empty() {continue};
                                for elem in rlocs[i].iter() {
                                    let result = closure(&bag_id, elem);
                                    if let Some(x) = result {
                                        locs[i].push(x)
                                    }
                                }
                            }
                        },
                        //works only with 1 element for bag
                        GridOption::READWRITE =>{
                            let mut locs = self.locs[self.write].borrow_mut();
                            let rlocs = self.locs[self.read].borrow();
                            // for each bag in read
                            for i in 0..rlocs.len() {
                                // calculate the bag_id
                                let bag_id = calculate_indexes_bag(i as i32, self.width, self.height).expect("error in calculate_indexes_bag");
                                // if the corresponding write bag is not empty
                                if !locs[i].is_empty() {
                                    // for each element in the write bag
                                    for elem in locs[i].iter_mut() {
                                        // apply the closure
                                        let result = closure(&bag_id, elem);
                                        if let Some(x) = result {
                                            *elem = x;
                                        }
                                    }
                                } else { // else if the corresponding bag is not empty
                                    // if the read bag is empty go to the next iteration
                                    if rlocs[i].is_empty() { continue }
                                    // for each element in the read bag
                                    for elem in rlocs[i].iter() {
                                        // apply the closure
                                        let result = closure(&bag_id, elem);
                                        if let Some(x) = result {
                                            // if the element is not already in the write bag
                                            if !locs[i].contains(&x){
                                                // push it
                                                locs[i].push(x);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                /// Return all the empty bags from read state.
                pub fn get_empty_bags(&self) -> Vec<Int2D>{
                    let mut empty_bags = Vec::new();
                    for i in 0 ..  self.width{
                        for j in 0 .. self.height{
                            let index = ((i * self.height) +j) as usize;
                            if self.locs[self.read].borrow()[index].is_empty() {
                                empty_bags.push(Int2D{x: i, y: j});
                            }
                        }
                    }
                    empty_bags
                }

                /// Return a random empty bag from read state. `None` if no bags are available.
                pub fn get_random_empty_bag(&self) -> Option<Int2D>{

                    let empty_bags = self.get_empty_bags();
                    if empty_bags.is_empty() {
                        return None;
                    }

                    let mut rng = rand::thread_rng();
                    let index = rng.gen_range(0..empty_bags.len());
                    Some(empty_bags[index])

                }


                            /// Return the position of the first element that matches the given value.
            /// Return None if no element matches.
            ///
            /// # Arguments
            /// * `value` - value to search for
            pub fn get_location(&self, object: &O) -> Option<Int2D> {
                let locs = self.locs[self.read].borrow();
                for i in  0..self.width{
                    for j in 0..self.height{
                        let index = (i *  self.height + j) as usize;
                        if locs[index].contains(object) {
                            return Some(Int2D {x: i, y: j });
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
            pub fn get_location_unbuffered(&self, object: &O) -> Option<Int2D> {
                let locs = self.locs[self.write].borrow();
                for i in  0..self.width{
                    for j in 0..self.height{
                        let index = (i *  self.height + j) as usize;
                        if locs[index].contains(object) {
                            return Some(Int2D {x: i, y: j });
                        }
                    }
                }
                None
            }

                /// Return all the objects in a specific position. `None` if position is empty.
                ///
                /// # Arguments
                /// * `loc` - location to get the objects
                pub fn get_objects(&self, loc: &Int2D) -> Option<Vec<O>> {
                    let mut obj = Vec::new();
                    let index = ((loc.x * self.height) + loc.y) as usize;
                    let rlocs = self.locs[self.read].borrow();
                    if rlocs[index].is_empty() {
                        None
                    } else {
                        for elem in &rlocs[index] {
                            obj.push(*elem);

                        }
                        Some(obj)
                    }
                }

                /// Return all the objects in a specific position from write state. `None` if position is empty.
                /// Useful when you want to get some object don't written in previous iterations, but into the current step.
                ///
                /// # Arguments
                /// * `loc` - location to get the objects
                pub fn get_objects_unbuffered(&self, loc: &Int2D) -> Option<Vec<O>> {

                    let mut obj = Vec::new();
                    let index = ((loc.x * self.height) + loc.y) as usize;
                    let locs = self.locs[self.write].borrow();

                    if locs[index].is_empty() {
                        None
                    } else {
                        for elem in &locs[index] {
                            obj.push(*elem);
                        }
                        Some(obj)
                    }
                }


                /// Iterate over the read state and apply the closure.
                ///
                /// # Arguments
                /// * `closure` - closure to apply to each element of the matrix
                pub fn iter_objects<F>(&self, closure: F)
                where
                    F: Fn(
                          &Int2D, //location
                          &O, //value
                    )
                {
                    for i in 0 .. self.width{
                        for j in 0 .. self.height{
                            let index = ((i * self.height) + j) as usize;
                            let locs = &self.locs[self.read].borrow()[index];
                            if !locs.is_empty() {
                                for obj in locs{
                                    closure(&Int2D{x: i, y: j}, obj);
                                }
                            }
                        }
                    }
                }

                /// Iterate over all objects inside the field and apply the closure.
                /// Useful when you want to access to all the objects changed/executed into the current step.
                ///
                /// # Arguments
                /// * `closure` - closure to apply to each element of the matrix
                pub fn iter_objects_unbuffered<F>(&self, closure: F)
                where
                    F: Fn(
                        &Int2D, //location
                        &O, //value
                    )
                {
                    for i in 0 ..  self.width{
                        for j in 0 .. self.height{
                            let index = ((i * self.height) + j) as usize;
                            let locs = &self.locs[self.write].borrow()[index];
                            let loc = Int2D{x: i, y: j};
                            if !locs.is_empty() {
                                for obj in locs{
                                    closure(&loc, obj);
                                }
                            }
                        }
                    }
                }


                /// Insert an object in a specific position.
                /// Double buffering swap the write and read state at the end of the step, so you have to call this function also if the object is not changed.
                ///
                /// If the position is empty, the value is pushed in the bag.
                /// If the position is not empty, the value is pushed in the bag and the old value is dropped.
                ///
                /// # Arguments
                /// * `obj` - object to insert
                /// * `loc` - location to insert the object
                pub fn set_object_location(&self, object: O, loc: &Int2D) {
                    let index = ((loc.x * self.height) + loc.y) as usize;
                    let mut locs = self.locs[self.write].borrow_mut();

                    if !locs[index].is_empty() {
                        locs[index].retain(|&obj| obj != object);
                    }

                    locs[index].push(object);
                }

                /// Remove an object from write state.
                /// You have to use it to remove an object written/updated in this step.
                /// Double buffering swap the write and read state at the end of the step, so you have to call
                /// this function only if the object was written/set in this step.
                ///
                /// # Arguments
                /// * `obj` - object to remove
                /// * `loc` - location to remove the object
                pub fn remove_object_location(&self, object: O, loc: &Int2D) {
                    let index = ((loc.x * self.height) + loc.y) as usize;
                    let mut locs = self.locs[self.write].borrow_mut();

                    if !locs[index].is_empty() {
                        locs[index].retain(|&obj| obj != object);
                    }
                }


            }

            impl<O: Eq + Hash + Clone + Copy> Field for DenseGrid2D<O> {
                /// Swap the state of the field and clear locs
                fn lazy_update(&mut self){
                    std::mem::swap(&mut self.read, &mut self.write);

                    let mut locs = self.locs[self.write].borrow_mut();
                    for i in 0..locs.len(){
                        locs[i].clear();
                    }
                }

                /// Swap the state of the field and updates the rlocs matrix
                fn update(&mut self) {
                    for i in 0 ..  self.width{
                        for j in 0 .. self.height{
                            let index = ((i * self.height) +j) as usize;
                            let value = self.locs[self.write].borrow_mut();
                            let mut r_value = self.locs[self.read].borrow_mut();
                            r_value.insert(index, value[index].clone());
                        }
                    }
                }
            }
        }
    }

#[allow(dead_code)]
fn calculate_indexes_bag(index: i32, width: i32, height: i32) -> Option<Int2D> {
    for i in 0..height {
        //check if the index parameter is in the row
        if index < (width * i) + width && index >= width * i {
            return Some(Int2D {
                x: index - width * i,
                y: i,
            });
        }
    }
    None
}
