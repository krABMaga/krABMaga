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
                    for obj in self.loc2objs.get_write(&*loc).unwrap().value_mut(){
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
                        .unwrap()
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
        pub struct SparseGrid2D<O: Eq + Hash + Clone + Copy> {
            pub locs: RefCell<HashMap<Int2D, Vec<O>>>,
            pub rlocs: RefCell<HashMap<Int2D, Vec<O>>>,
            pub width: i32,
            pub height: i32,
        }
        impl<O: Eq + Hash + Clone + Copy> SparseGrid2D<O> {
            pub fn new(width: i32, height: i32) -> SparseGrid2D<O> {
                SparseGrid2D {
                    locs: RefCell::new(HashMap::new()),
                    rlocs: RefCell::new(HashMap::new()),
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
                        let mut rlocs = self.rlocs.borrow_mut();
                        for (key,value) in rlocs.iter_mut() {
                            for obj in value{
                                *obj = closure(key, obj).unwrap();
                            }
                        }
                    },
                    GridOption::WRITE => {
                        let mut locs = self.locs.borrow_mut();
                        for (key,value) in locs.iter_mut() {
                            for obj in value{
                                *obj = closure(key, obj).unwrap();
                            }
                        }
                    }
                    // TO CHECK
                    //works only with 1 element for bag
                    GridOption::READWRITE =>{
                        let rlocs = self.rlocs.borrow();
                        let mut locs = self.locs.borrow_mut();

                        // for each bag in read
                        for (key, value) in rlocs.iter() {
                            if let Some(write_value) = locs.get_mut(key){
                                for obj in write_value{
                                    *obj = closure(key, obj).unwrap();
                                }
                            }else{
                                for obj in value{
                                    let new_bag = vec![closure(key, obj).unwrap()];
                                    locs.insert(*key, new_bag);
                                }
                            }
                        }
                    }
                }
            }

            pub fn get_objects(&self, loc: &Int2D) -> Option<Vec<O>> {
                match self.rlocs.borrow().get(loc) {
                    Some(obj) => {
                        Some(obj.clone())
                    },
                    None => None,
                }
            }

            pub fn get_objects_unbuffered(&self, loc: &Int2D) -> Option<Vec<O>> {
                match self.locs.borrow().get(loc) {
                    Some(obj) => {
                        Some(obj.clone())
                    },
                    None => None,
                }
            }


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



            // pub fn remove_object(&self, object: &O) {
            //     if let Some(old_loc) = self.locs.get(object) {
            //         self.locs_inversed
            //             .get_mut(old_loc)
            //             .unwrap()
            //             .value_mut()
            //             .retain(|&x| x != *object);
            //     }

            //     self.locs.remove(object);
            // }


            // pub fn remove_object(&self, object: &O) {
            //  if let Some(old_loc) = self.locs.get(object) {
            //     self.locs_inversed
            //         .get_mut(old_loc)
            //         .unwrap()
            //         .value_mut()
            //         .retain(|&x| x != *object);
            // }

            //     let loc = self.a2loc.borrow().get(object).unwrap();
            //     let index = ((loc.x * self.height) + loc.y) as usize;
            //     for
            //     match self.locs.try_borrow_mut() {
            //         Ok(mut locs) => {
            //             locs[index].retain(|&x| x != *object);
            //         },
            //         Err(_) => {},
            //     }
            //     match self.rlocs.try_borrow_mut() {
            //         Ok(mut locs) => {
            //             locs[index].retain(|&x| x != *object);
            //         },
            //         Err(_) => {},
            //     }
            // }

            // pub fn get_object(&self, object: &O) -> Option<&O> {
            //     match self.locs.get_key_value(object) {
            //         Some((updated_object, _loc)) => Some(updated_object),
            //         None => None,
            //     }
            // }

            // pub fn get_object_location(&self, object: O) -> Option<&Int2D> {
            //     self.locs.get(&object)
            // }
        }

        impl<O: Eq + Hash + Clone + Copy> Field for SparseGrid2D<O> {
            fn lazy_update(&mut self){
                unsafe {
                    std::ptr::swap(
                        self.rlocs.as_ptr(),
                        self.locs.as_ptr(),
                    )
                }
                self.locs.borrow_mut().clear();
            }

            fn update(&mut self) {
                let mut rlocs = self.rlocs.borrow_mut();
                for (key, value) in self.locs.borrow().iter() {
                    rlocs.insert(*key, value.clone());
                }
            }
        }
    }
}
