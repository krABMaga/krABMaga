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
            // pub locs: RefCell<HashMap<Int2D, Vec<O>>>,
            /// Hashmap to read data. Key is the value, value is the location.
            // pub rlocs: RefCell<HashMap<Int2D, Vec<O>>>,
            pub locs: Vec<RefCell<HashMap<Int2D, Vec<O>>>>,
            read: usize,
            write: usize,
            /// First dimension of the field
            pub width: i32,
            /// Second dimension of the field
            pub height: i32,
        }
        impl<O: Eq + Hash + Clone + Copy> SparseGrid2D<O> {
            /// create a new instance of SparseGrid2D
            /// # Arguments
            ///
            /// * `width` - first dimension of the field
            /// * `height` - second dimension of the field
            pub fn new(width: i32, height: i32) -> SparseGrid2D<O> {
                SparseGrid2D {
                    // locs: RefCell::new(HashMap::new()),
                    // rlocs: RefCell::new(HashMap::new()),
                    locs: vec![RefCell::new(HashMap::new()), RefCell::new(HashMap::new())],
                    read: 0,
                    write: 1,
                    width,
                    height,
                }
            }

            /// Apply a closure to all objects.
            /// You have to return an object.
            /// You can return the same object or a new/updated one or `None` to remove it.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to all objects
            /// * `option` - option to read or write
            /// ## `option` possible variants
            /// * `READ` - update the objects from rlocs
            /// * `WRITE` - update the objects from locs
            /// * `READWRITE` - check locs and rlocs simultaneously to apply the closure
            ///
            /// # Example
            ///
            /// ```
            /// struct Object{
            ///     id: i32,
            ///     flag: bool,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// for i in 0..10 {
            ///    for j in 0..10 {
            ///       grid.set_object_location(Object::new(i*10 + j), &Int2D::new(i, j));
            ///    }
            /// }
            ///
            /// grid.apply_to_all_values(|loc, obj| {
            ///     let mut obj = *obj
            ///     obj.flag = true;
            ///     Some(obj)
            /// }, GridOption::WRITE); // Or READWRITE
            ///
            /// grid.lazy_update();
            ///
            /// grid.apply_to_all_values(|loc, obj| {
            ///     assert!(obj.flag);
            ///     None    // return None to delete object
            /// }, GridOption::READ);  // Or READWRITE
            ///
            /// ```
            ///
            pub fn apply_to_all_values<F>(&self, closure: F, option: GridOption)
            where
                F: Fn(&Int2D, &O) -> Option<O>,
            {
                match option {
                    GridOption::READ => {
                        let mut rlocs = self.locs[self.read].borrow_mut();
                        for (key,value) in rlocs.iter_mut() {
                            for obj in value{
                                *obj = closure(key, obj).expect("error on closure");
                            }
                        }
                    },
                    GridOption::WRITE => {
                        let mut locs = self.locs[self.write].borrow_mut();
                        for (key,value) in locs.iter_mut() {
                            for obj in value{
                                *obj = closure(key, obj).expect("error on closure");
                            }
                        }
                    }
                    // TO CHECK
                    // works only with 1 element for bag
                    GridOption::READWRITE =>{
                        let rlocs = self.locs[self.read].borrow();
                        let mut locs = self.locs[self.write].borrow_mut();

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

            /// Return the position of the first element that matches the given value.
            /// Return None if no element matches.
            ///
            /// # Arguments
            /// * `value` - value to search for
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///  id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<u16>::new(10, 10);
            /// grid.set_object_location(Object::new(1), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(1), &Int2D::new(6, 6));
            ///
            /// grid.lazy_update();
            /// let pos = grid.get_location(&Object::new(1));
            /// assert_eq!(pos, Some(Int2D::new(5, 5)));
            ///
            /// let none = grid.get_location(&Object::new(3));
            /// assert_eq!(none, None);
            /// ```
            ///
            pub fn get_location(&self, object: &O) -> Option<Int2D> {
                let rlocs = self.locs[self.read].borrow();
                for (key, objs) in rlocs.iter() {
                    for obj in objs {
                        if *obj == *object {
                            return Some(*key);
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
            /// struct Object{
            ///  id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<u16>::new(10, 10);
            /// grid.set_object_location(Object::new(1), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(1), &Int2D::new(6, 6));
            ///
            /// // Work on write state, so on unupdated state
            /// let pos = grid.get_location_unbuffered(&Object::new(1));
            /// assert_eq!(pos, Some(Int2D::new(5, 5)));
            ///
            /// let none = grid.get_location_unbuffered(&Object::new(2));
            /// assert_eq!(none, None);
            ///
            /// grid.lazy_update();
            /// let pos = grid.get_location_unbuffered(&Object::new(1));
            /// assert_eq!(pos, None);
            /// ```
            ///
            pub fn get_location_unbuffered(&self, object: &O) -> Option<Int2D> {
                let locs = self.locs[self.write].borrow();
                for (key, objs) in locs.iter() {
                    for obj in objs {
                        if *obj == *object {
                            return Some(*key);
                        }
                    }
                }
                None
            }

            /// Return all the objects in a specific position. `None` if position is empty.
            ///
            /// # Arguments
            /// * `loc` - location to get the objects
            ///
            /// # Example
            /// ```
            /// struct Object{
            /// id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// grid.set_object_location(Object::new(1), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(2), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(3), &Int2D::new(5, 5));
            ///
            /// grid.lazy_update();
            /// let objects = grid.get_objects_at(&Int2D::new(5, 5));
            /// assert_eq!(objects.unwrap().len(), 3);
            ///
            /// let none = grid.get_objects_at(&Int2D::new(6, 6));
            /// assert_eq!(none, None);
            /// ```
            pub fn get_objects(&self, loc: &Int2D) -> Option<Vec<O>> {
                self.locs[self.read].borrow().get(loc).cloned()
            }

            /// Return all the objects in a specific position from write state. `None` if position is empty.
            /// Useful when you want to get some object don't written in previous iterations, but into the current step.
            ///
            /// # Arguments
            /// * `loc` - location to get the objects
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///     id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// grid.set_object_location(Object::new(1), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(2), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(3), &Int2D::new(5, 5));
            ///
            /// let objects = grid.get_objects_at_unbuffered(&Int2D::new(5, 5));
            /// assert_eq!(objects.unwrap().len(), 3);
            ///
            /// grid.lazy_update();
            /// let none = grid.get_objects_at_unbuffered(&Int2D::new(5, 5));
            /// assert_eq!(none, None);
            ///
            /// ```
            pub fn get_objects_unbuffered(&self, loc: &Int2D) -> Option<Vec<O>> {
                self.locs[self.write].borrow().get(loc).cloned()
            }


            /// Return all the empty bags from read state.
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///    id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// let empty = grid.get_empty_bags();
            /// assert_eq!(empty.len(), 100);
            ///
            /// for i in 0..10 {
            ///   for j in 0..10 {
            ///      grid.set_object_location(Object::new(i*10 + j), &Int2D::new(i, j));
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
            /// ```
            pub fn get_empty_bags(&self) -> Vec<Int2D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let loc = Int2D{x: i, y: j};
                        match self.locs[self.read].borrow().get(&loc){
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


            /// Return a random empty bag from read state. `None` if no bags are available.
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///   id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// let empty = grid.get_random_empty_bag();
            /// assert(empty.is_some());
            ///
            /// grid.set_object_location(Object::new(1), &empty.unwrap());
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

            /// Iterate over the read state and apply the closure.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to each element of the matrix
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///    id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// for i in 0..10{
            ///    for j in 0..10{
            ///       grid.set_object_location(Object::new(i * j), &Int2D::new(i, j));
            ///    }
            /// }
            ///
            /// grid.lazy_update();
            /// grid.iter_objects(|loc, obj| {
            ///     assert_eq!(loc.x * loc.y, obj.id);
            ///     // Do something
            /// });
            ///
            /// ```
            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &O//value
                )
            {
                let rlocs = self.locs[self.read].borrow();
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
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///   id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// for i in 0..10{
            ///     for j in 0..10{
            ///         grid.set_object_location(Object::new(i * j), &Int2D::new(i, j));
            ///     }
            /// }
            ///
            /// grid.iter_objects_unbuffered(|loc, obj| {
            ///     assert_eq!(loc.x * loc.y, obj.id);
            ///     // Do something
            /// });
            ///
            /// ```
            pub fn iter_objects_unbuffered<F>(&self, closure: F)
            where
                F: Fn(
                    &Int2D, //location
                    &O, //value
                )
            {
                let locs = self.locs[self.write].borrow();
                for (key, bag) in locs.iter(){
                    for obj in bag{
                        closure(key, obj);
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
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///    id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            ///
            /// grid.set_object_location(Object::new(1), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(2), &Int2D::new(5, 5));
            ///
            /// let none = grid.get_objects(&Int2D::new(5, 5));
            /// assert_eq!(none, None);
            ///
            /// grid.lazy_update();
            /// let objects = grid.get_objects(&Int2D::new(5, 5));
            /// assert_eq!(objects.unwrap().len(), 2);
            /// assert_eq!(objects.unwrap()[0].id, 1);
            /// assert_eq!(objects.unwrap()[1].id, 2);
            ///
            /// ```
            ///
            pub fn set_object_location(&self, object: O, loc: &Int2D) {
                let mut locs = self.locs[self.write].borrow_mut();
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
            ///
            /// # Example
            /// ```
            /// struct Object{
            ///   id: i32,
            /// }
            ///
            /// let mut grid = SparseGrid2D::<Object>::new(10, 10);
            /// grid.set_object_location(Object::new(1), &Int2D::new(5, 5));
            /// grid.set_object_location(Object::new(2), &Int2D::new(5, 5));
            ///
            /// grid.remove_object_location(&Object::new(1), &Int2D::new(5, 5));
            /// let objects = grid.get_objects_unbuffered(&Int2D::new(5, 5));
            /// assert_eq!(objects.unwrap().len(), 1);
            /// assert_eq!(objects.unwrap()[0].id, 2);
            ///
            /// grid.lazy_update();
            /// let objects = grid.get_objects(&Int2D::new(5, 5));
            /// assert_eq!(objects.unwrap().len(), 1);
            /// assert_eq!(objects.unwrap()[0].id, 2);
            ///
            /// ```
            pub fn remove_object_location(&self, object: O, loc: &Int2D) {
                let mut locs = self.locs[self.write].borrow_mut();
                let bag = locs.get_mut(loc);
                if let Some(bag) = bag {
                    bag.retain(|&obj| obj != object);
                    if bag.is_empty(){
                        locs.remove(loc);
                    }
                }
            }

        }

        impl<O: Eq + Hash + Clone + Copy> Field for SparseGrid2D<O> {
            /// Swap the state of the field and clear locs
            fn lazy_update(&mut self){
                std::mem::swap(&mut self.read, &mut self.write);
                self.locs[self.write].borrow_mut().clear();
            }

            /// Swap the state of the field and updates the rlocs matrix
            fn update(&mut self) {
                let mut rlocs = self.locs[self.read].borrow_mut();
                rlocs.clear();
                for (key, value) in self.locs[self.write].borrow().iter() {
                    rlocs.insert(*key, value.clone());
                }
                self.locs[self.write].borrow_mut().clear();
            }
        }
    }
}
