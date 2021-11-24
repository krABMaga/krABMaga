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

        pub struct DenseNumberGrid2D<T: Copy + Clone + PartialEq> {
            pub locs: RefCell<Vec<Vec<T>>>,
            pub rlocs: RefCell<Vec<Vec<T>>>,
            pub width: i32,
            pub height: i32
        }

        impl<T: Copy + Clone + PartialEq> DenseNumberGrid2D<T> {
            pub fn new(width: i32, height: i32) -> DenseNumberGrid2D<T> {
                DenseNumberGrid2D {
                    locs: RefCell::new(std::iter::repeat_with(Vec::new).take((width * height) as usize).collect()),
                    rlocs: RefCell::new(std::iter::repeat_with(Vec::new).take((width * height)as usize).collect()),
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
                        let mut rlocs = self.rlocs.borrow_mut();
                        for i in 0 .. rlocs.len() {
                            let mut vec = Vec::new();
                            if rlocs[i].is_empty() {continue};
                            for elem in rlocs[i].iter() {
                                let result = closure(elem);
                                vec.push(result);
                            }
                            rlocs[i] = vec;
                        }
                    },
                    GridOption::WRITE => {
                        let mut locs = self.locs.borrow_mut();
                        let rlocs = self.rlocs.borrow();
                        for i in 0 .. rlocs.len() {
                            if rlocs[i].is_empty() {continue};
                            for elem in rlocs[i].iter() {
                                let result = closure(elem);
                                locs[i].push(result);
                            }
                        }
                    },
                    //works only with 1 element for bag
                    GridOption::READWRITE =>{
                        let mut locs = self.locs.borrow_mut();
                        let rlocs = self.rlocs.borrow_mut();
                        // for each bag in read
                        for i in 0..rlocs.len() {
                            // calculate the bag_id
                            // if the corresponding write bag is not empty
                            if !locs[i].is_empty() {
                                // for each element in the write bag
                                for elem in locs[i].iter_mut() {
                                    // apply the closure
                                    let result = closure(elem);
                                    *elem = result;
                                }
                            }else{ // else if the corresponding bag is not empty
                                // if the read bag is empty go to the next iteration
                                if rlocs[i].is_empty() { continue }
                                // for each element in the read bag
                                for elem in rlocs[i].iter() {
                                    // apply the closure
                                    let result = closure(elem);
                                    if !locs[i].contains(&result){
                                        // push it
                                        locs[i].push(result);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            pub fn get_empty_bags(&self) -> Vec<Int2D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let index = ((i * self.height) +j) as usize;
                        if self.rlocs.borrow()[index].is_empty() {
                            empty_bags.push(Int2D{x: i, y: j});
                        }
                    }
                }
                empty_bags
            }

            pub fn get_random_empty_bag(&self) -> Option<Int2D>{
                let mut rng = rand::thread_rng();
                loop {
                    let i = rng.gen_range(0..self.width);
                    let j = rng.gen_range(0..self.height);
                    let loc = Int2D{x: i, y: j};
                    let index = ((i * self.height) +j) as usize;
                    if self.rlocs.borrow()[index].is_empty() {
                        return Some(loc);
                    }
                }
            }

            pub fn get_value(&self, loc: &Int2D) -> Option<T> {
                let index = ((loc.x * self.height) + loc.y) as usize;
                let rlocs = self.rlocs.borrow();
                if rlocs[index].is_empty() {
                    None
                } else {
                    Some(rlocs[index][0])
                }
            }

            pub fn get_value_unbuffered(&self, loc: &Int2D) -> Option<Vec<T>> {
                let mut obj = Vec::new();
                let index = ((loc.x * self.height) + loc.y) as usize;
                let locs = self.locs.borrow();

                if locs[index].is_empty() {
                    None
                } else {
                    for elem in &locs[index] {
                        obj.push(*elem);
                    }
                    Some(obj)
                }
            }

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
                        let locs = &self.rlocs.borrow()[index];
                        if !locs.is_empty() {
                            for obj in locs{
                                closure(&Int2D{x: i, y: j}, obj);
                            }
                        }
                    }
                }
            }

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
                        let locs = &self.locs.borrow()[index];
                        let loc = Int2D{x: i, y: j};
                        if !locs.is_empty() {
                            for obj in locs{
                                closure(&loc, obj);
                            }
                        }
                    }
                }
            }



            pub fn set_value_location(&self, value: T, loc: &Int2D) {
                let index = ((loc.x * self.height) + loc.y) as usize;
                let mut locs = self.locs.borrow_mut();

                if !locs[index].is_empty() {
                    locs[index].retain(|&obj| obj != value);
                }

                locs[index].push(value);
            }

        }

        impl<T: Copy + Clone + PartialEq> Field for DenseNumberGrid2D<T> {
            fn lazy_update(&mut self){
                unsafe {
                    std::ptr::swap(
                        self.locs.as_ptr(),
                        self.rlocs.as_ptr(),
                    )
                }
                let mut locs = self.locs.borrow_mut();
                for i in 0..locs.len(){
                    locs[i].clear();
                }
            }

            fn update(&mut self) {
                for i in 0 ..  self.width{
                    for j in 0 .. self.height{
                        let index = ((i * self.height) +j) as usize;
                        let value = self.locs.borrow_mut();
                        let mut r_value = self.rlocs.borrow_mut();
                        r_value.insert(index, value[index].clone());
                    }
                }
            }
        }

    }
}
