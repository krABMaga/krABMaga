use crate::engine::{
    fields::{field::Field, grid_option::GridOption},
    location::Int2D,
};
use crate::rand::Rng;

use cfg_if::cfg_if;
use std::hash::Hash;

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
        pub struct SparseGrid2D<O: Eq + Hash + Clone + Copy> {
            pub obj2loc: DBDashMap<O, Int2D>, // old locs
            pub loc2objs: DBDashMap<Int2D, Vec<O>>, // old locs_inversed
            pub width: i32,
            pub height: i32,
        }

        impl<O: Eq + Hash + Clone + Copy> SparseGrid2D<O> {
            pub fn new(width: i32, height: i32) -> SparseGrid2D<O> {
                SparseGrid2D {
                    obj2loc: DBDashMap::with_capacity((width * height) as usize),
                    loc2objs: DBDashMap::with_capacity((width * height) as usize),
                    width,
                    height,
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

            pub fn get_empty_bags(&self) -> Vec<Int2D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        match self.loc2objs.get_read(&Int2D{x: i, y: j}){
                            Some(_x) => { },
                            None => {empty_bags.push(Int2D{x: i, y: j})}
                        }
                    }
                }
                empty_bags
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

            pub fn get_unbuffered(&self, object: &O) -> Option<O> {
                match self.obj2loc.get_write(object){
                    Some(loc) =>{
                    for obj in self.loc2objs.get_write(&*loc).expect("error on get_write").value_mut(){
                        if obj == object {
                            return Some(*obj);
                        }
                    }
                    }, None =>{
                        return None;
                    }
                }
                None
            }

            pub fn get_random_empty_bag(&self) -> Option<Int2D>{
                let mut rng = rand::thread_rng();
                loop {
                    let loc = Int2D{x: rng.gen_range(0..self.width), y: rng.gen_range(0..self.height)};
                    match self.loc2objs.get_read(&loc){
                        Some(_bag) =>{},
                        None => {
                            return Some(loc)
                        }
                    }
                }
            }

            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &O, //value
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
                        .expect("error on get_write")
                        .value_mut()
                        .retain(|&x| x != *object);
                }

                self.obj2loc.remove(object);
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

        impl<O: Eq + Hash + Clone + Copy> Field for SparseGrid2D<O> {
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

        /// Field with double buffering for sparse matrix
        pub struct SparseGrid2D<O: Eq + Hash + Clone + Copy> {
            /// Hashmap to write data. Key is location, value is the number.
            pub locs: RefCell<HashMap<Int2D, Vec<O>>>,
            /// Hashmap to read data. Key is the value, value is the location.
            pub rlocs: RefCell<HashMap<Int2D, Vec<O>>>,
            /// First dimension of the field
            pub width: i32,
            /// Second dimension of the field
            pub height: i32,
        }
        impl<O: Eq + Hash + Clone + Copy> SparseGrid2D<O> {
            /// create a new instance of SparseNumberenseGrid2D
            /// # Arguments
            ///
            /// * `width` - first dimension of the field
            /// * `height` - second dimension of the field
            pub fn new(width: i32, height: i32) -> SparseGrid2D<O> {
                SparseGrid2D {
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
                F: Fn(&Int2D, &O) -> Option<O>,
            {
                match option {
                    GridOption::READ => {
                        let mut rlocs = self.rlocs.borrow_mut();
                        for (key,value) in rlocs.iter_mut() {
                            for obj in value{
                                *obj = closure(key, obj).expect("error on closure");
                            }
                        }
                    },
                    GridOption::WRITE => {
                        let mut locs = self.locs.borrow_mut();
                        for (key,value) in locs.iter_mut() {
                            for obj in value{
                                *obj = closure(key, obj).expect("error on closure");
                            }
                        }
                    }
                    // TO CHECK
                    // works only with 1 element for bag
                    GridOption::READWRITE =>{
                        let rlocs = self.rlocs.borrow();
                        let mut locs = self.locs.borrow_mut();

                        // for each bag in read
                        for (key, value) in rlocs.iter() {
                            if let Some(write_value) = locs.get_mut(key){
                                for obj in write_value{
                                    *obj = closure(key, obj).expect("error on closure");
                                }
                            }else{
                                for obj in value{
                                    let new_bag = vec![closure(key, obj).expect("error on closure")];
                                    locs.insert(*key, new_bag);
                                }
                            }
                        }
                    }
                }
            }

            /// Return the position of the first element that matches the given object.
            /// Return None if no element matches.
            ///
            /// # Arguments
            /// * `value` - value to search for
            pub fn get_location(&self, object: &O) -> Option<Int2D> {
                let rlocs = self.rlocs.borrow();
                for (key, objs) in rlocs.iter() {
                    for obj in objs {
                        if *obj == *object {
                            return Some(*key);
                        }
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
            pub fn get_location_unbuffered(&self, object: &O) -> Option<Int2D> {
                let locs = self.locs.borrow();
                for (key, objs) in locs.iter() {
                    for obj in objs {
                        if *obj == *object {
                            return Some(*key);
                        }
                    }
                }
                None
            }

            /// Get the object at a specific location.
            ///
            /// # Arguments
            /// * `loc` - location to get the ogject from
            pub fn get_objects(&self, loc: &Int2D) -> Option<Vec<O>> {
                self.rlocs.borrow().get(loc).cloned()
            }

            /// Get the object at a specific location from the write state.
            ///
            /// # Arguments
            /// * `loc` - location to get the object from
            pub fn get_objects_unbuffered(&self, loc: &Int2D) -> Option<Vec<O>> {
                self.locs.borrow().get(loc).cloned()
            }


            /// Get all empty bags from read state.
            pub fn get_empty_bags(&self) -> Vec<Int2D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let loc = Int2D{x: i, y: j};
                        match self.rlocs.borrow().get(&loc){
                            Some(_bag) =>{
                                if _bag.is_empty(){
                                    empty_bags.push(loc);
                                }
                            },
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

            /// Iterate over the Read State matrix and call the closure for each object.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to all objects
            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &O//value
                )
            {
                let rlocs = self.rlocs.borrow();
                for (key, bag) in rlocs.iter(){
                    for obj in bag{
                        closure(key, obj);
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
                let locs = self.locs.borrow();
                for (key, bag) in locs.iter(){
                    for obj in bag{
                        closure(key, obj);
                    }
                }
            }

            /// Insert an object in a specific position.
            /// Double buffering swap the write and read state at the end of the step, so you have to call this function also if the object is not changed.
            ///
            /// If the position is empty, the object is pushed in the bag.
            /// If the position is not empty, the object is pushed in the bag and the old object is dropped.
            ///
            /// # Arguments
            /// * `loc` - location to set the object at
            /// * `object` - object to insert
            pub fn set_object_location(&self, object: O, loc: &Int2D) {
                let mut locs = self.locs.borrow_mut();
                match locs.get_mut(loc){
                    Some(bag) =>{
                        bag.push(object);
                    },
                    None =>{
                        locs.insert(*loc, [object].to_vec());
                    }
                }
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
                let mut locs = self.locs.borrow_mut();
                let bag = locs.get_mut(loc);
                if let Some(bag) = bag {
                    bag.retain(|&obj| obj != object);
                }
            }

        }

        impl<O: Eq + Hash + Clone + Copy> Field for SparseGrid2D<O> {
            /// Swap the state of the field and clear locs
            fn lazy_update(&mut self){
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
                    rlocs.insert(*key, value.clone());
                }
            }
        }
    }
}
