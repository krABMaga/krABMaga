use crate::engine::{
    fields::field::Field,
    location::{Int3D, Real3D},
};

use core::fmt::Display;
use std::cmp;
use std::hash::Hash;
/// A trait to request implementation of the two basic function that must be implemented
pub trait Location3D<Real3D> {
    fn get_location(self) -> Real3D;
    fn set_location(&mut self, loc: Real3D);
}

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "visualization", feature = "visualization_wasm", feature = "parallel"))] {
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use std::cell::RefCell;
        use crate::*;
    }
}

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        pub struct Field3D<O: Location3D<Real3D> + Clone + Hash + Eq + Copy + Display> {
            pub findex: DBDashMap<O, Int3D>,
            pub fbag: DBDashMap<Int3D, Vec<O>>,
            pub floc: DBDashMap<O, Real3D>,
            pub width: f32,
            pub height: f32,
            pub length: f32,
            pub discretization: f32,
            pub toroidal: bool,
        }

        //field 2d
        impl<O: Location3D<Real3D> + Clone + Hash + Eq + Copy + Display> Field3D<O> {
            pub fn new(w: f32, h: f32, l: f32, d: f32, t: bool) -> Field3D<O> {
                Field3D {
                    findex: DBDashMap::new(),
                    fbag: DBDashMap::new(),
                    floc: DBDashMap::new(),
                    width: w,
                    height: h,
                    length: l,
                    discretization: d,
                    toroidal: t,
                }
            }

            fn discretize(&self, loc: &Real3D) -> Int3D {
                let x_floor = (loc.x/self.discretization).floor();
                let x_floor = x_floor as i32;

                let y_floor = (loc.y/self.discretization).floor();
                let y_floor = y_floor as i32;

                let z_floor = (loc.z/self.discretization).floor();
                let z_floor = z_floor as i32;

                Int3D {
                    x: x_floor,
                    y: y_floor,
                    z: z_floor,
                }
            }

            pub fn get_neighbors_within_distance(&self, loc: Real3D, dist: f32) -> Vec<O> {

                //TODO correct?
                let density = ((self.width * self.height * self.length) as usize)/(self.findex.r_len());
                let sdist = (dist * dist * dist) as usize;
                let mut neighbors: Vec<O> = Vec::with_capacity(density as usize * sdist);

                if dist <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (dist/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.height/self.discretization).ceil() as i32;
                let max_z =  (self.length/self.discretization).ceil() as i32;

                let mut min_i = disc_loc.x - disc_dist;
                let mut max_i = disc_loc.x + disc_dist;
                let mut min_j = disc_loc.y - disc_dist;
                let mut max_j = disc_loc.y + disc_dist;
                let mut min_k = disc_loc.z - disc_dist;
                let mut max_k = disc_loc.z + disc_dist;

                if self.toroidal {
                    min_i = cmp::max(0, min_i);
                    max_i = cmp::min(max_i, max_x-1);
                    min_j = cmp::max(0, min_j);
                    max_j = cmp::min(max_j, max_y-1);
                    min_k = cmp::max(0, min_k);
                    max_k = cmp::min(max_k, max_z-1);
                }

                for i in min_i..max_i+1 {
                    for j in min_j..max_j+1 {
                        for k in min_k..max_k+1 {
                            //TODO 
                        }
                        // let bag_id = Int3D {
                        //     x: t_transform(i, max_x),
                        //     y: t_transform(j, max_y),
                        // };
                        // let check = check_circle(&bag_id, self.discretization, self.width, self.height, &loc, dist, self.toroidal);
                        // let vector =  match self.fbag.get_read(&bag_id) {
                        //     Some(i) => i,
                        //     None => continue,
                        // };

                        // for elem in vector{
                        //     if (check == 0 &&
                        //         distance(&loc, &(elem.get_location()), self.width, self.height, self.toroidal) <= dist) ||
                        //         check == 1
                        //     {
                        //         neighbors.push(*elem);
                        //     }
                        // }
                    }
                }
                neighbors
            }

            pub fn get_neighbors_within_relax_distance(&self, loc: Real3D, dist: f32) -> Vec<O> {

                let density = ((self.width * self.height * self.length) as usize)/(self.findex.r_len());
                let sdist = (dist * dist * dist) as usize;
                let mut neighbors: Vec<O> = Vec::with_capacity(density as usize * sdist);

                if dist <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (dist/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.height/self.discretization).ceil() as i32;
                let max_z =  (self.length/self.discretization).ceil() as i32;

                let mut min_i = disc_loc.x - disc_dist;
                let mut max_i = disc_loc.x + disc_dist;
                let mut min_j = disc_loc.y - disc_dist;
                let mut max_j = disc_loc.y + disc_dist;
                let mut min_k = disc_loc.z - disc_dist;
                let mut max_k = disc_loc.z + disc_dist;

                if self.toroidal {
                    min_i = cmp::max(0, min_i);
                    max_i = cmp::min(max_i, max_x-1);
                    min_j = cmp::max(0, min_j);
                    max_j = cmp::min(max_j, max_y-1);
                    min_k = cmp::max(0, min_k);
                    max_k = cmp::min(max_k, max_z-1);
                }

                for i in min_i..max_i+1 {
                    for j in min_j..max_j+1 {
                        for k in min_k..max_k+1 {
                            let bag_id = Int3D {
                                x: t_transform(i, max_x),
                                y: t_transform(j, max_y),
                                z: t_transform(k, max_z),
                            };
                            let vector =  match self.fbag.get_read(&bag_id) {
                                Some(i) => i,
                                None => continue,
                            };
                            for elem in vector {
                                neighbors.push(*elem);
                            }
                        }
                        
                    }
                }
                neighbors
            }

            // take an object and check if it is in the field
            // if so return the object
            // mainly used for visualization
            pub fn get(&self, object: &O) -> Option<&O> {
                match self.floc.get_key_value(object) {
                    Some((updated_object, _loc)) => Some(updated_object),
                    None => None,
                }
            }

            // take a location and return the corresponding objects on that location
            pub fn get_objects(&self, loc: Real3D) -> Vec<&O> {
                let bag = self.discretize(&loc);
                let mut result = Vec::new();

                match self.fbag.get_read(&bag){
                    Some(v) => {
                        for el in v{
                            result.push(el);
                        }
                    }
                    None => ()
                }
                result
            }

            // take an object and return the corresponding location
            pub fn get_location(&self, object: O) -> Option<&Real3D> {
                self.floc.get_read(&object)
            }

            // take an object and return the corresponding location from the write state
            pub fn get_location_unbuffered(&self, object: O) -> Option<Real3D> {
                let mut loc = self.floc.get_write(&object).expect("error on get_write");
                Some(*loc.value_mut())
            }

            // take an object and check if it is in the field
            // if so return the object from the write bags
            // mainly used for visualization
            pub fn get_unbuffered(&self, object: &O) -> Option<O> {

                match self.floc.get_write(object){
                    Some(loc) =>{
                        let real_loc = self.discretize(&loc);
                        for obj in self.fbag.get_write(&real_loc).expect("error on get_write").value_mut(){
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

            // return the number of objects in the field
            pub fn num_objects(&self) -> usize {
                self.findex.r_len()
            }

            // return the number of objects in the field on that location
            pub fn num_objects_at_location(&self, loc: Real3D) -> usize {
                let bag = self.discretize(&loc);
                match self.fbag.get_read(&bag){
                    Some(v) => {
                        v.len()
                    }
                    None => 0
                }
            }

            // put the object in that location
            pub fn set_object_location(&self, object: O, loc: Real3D) {
                let bag = self.discretize(&loc);
                self.floc.insert(object, loc);
                self.findex.insert(object, bag);
                match self.fbag.get_write(&bag){
                    Some(v) => {
                            let mut v = v;
                            v.push(object);
                    }
                    None => {
                            let mut v = Vec::new();
                            v.push(object);
                            self.fbag.insert(bag,v);
                    }
                };
            }

        }

        impl<O: Location3D<Real3D> + Clone + Hash + Eq + Copy + Display> Field for Field3D<O>{
            fn update(&mut self){
                self.floc.update();
                self.fbag.update();
                self.findex.update();
            }
            fn lazy_update(&mut self){
                self.floc.lazy_update();
                self.fbag.lazy_update();
                self.findex.lazy_update();
            }
        }
    } else {
        ///  Sparse matrix structure modelling agent interactions on a 2D real space with coordinates represented by 2D f64 tuples
        pub struct Field3D<O: Location3D<Real3D> + Clone + Hash + Eq + Copy + Display> {
            /// Matrix to write data. Vector of vectors that have a generic Object O inside
            pub bags: RefCell<Vec<Vec<O>>>,
            /// Matrix to read data. Vector of vectors that have a generic Object O inside
            pub rbags: RefCell<Vec<Vec<O>>>,
            /// Number of agents inside the field
            pub nagents: RefCell<usize>,
            /// First dimension of the field
            pub width: f32,
            /// Second dimension of the field
            pub height: f32,
            /// Third dimension of the field
            pub length: f32,
            /// Value to discretize `Real3D` positions to our Matrix
            pub discretization: f32,
            /// `true` if you want a Toroidal field, `false` otherwise
            pub toroidal: bool,
            /// Discretized height of the field
            pub dh: i32,
            /// Discretized width of the field
            pub dw: i32,
            pub dz: i32,
            /// Field density
            pub density_estimation:usize,
            /// `true` if you want calculate field density, `false` otherwise
            pub density_estimation_check:bool,
        }
        impl<O: Location3D<Real3D> + Clone + Hash + Eq + Copy + Display> Field3D<O>  {

            /// Create a new `Field3D`
            ///
            /// # Arguments
            /// * `w` - Width, first dimension of the field
            /// * `h` - Height, second dimension of the field
            /// * `d` - Value to discretize `Real3D` positions to our Matrix
            /// * `t` - `true` if you want a Toroidal field, `false` otherwise
            /// 
            // RefCell::new(std::iter::repeat_with(Vec::new).take((((w/d).ceil()+1.0) * ((h/d).ceil() +1.0))as usize).collect()),

            pub fn new(w: f32, h: f32, l: f32, d: f32, t: bool) -> Field3D<O> {
                Field3D {
                    // bags: RefCell::new(std::iter::repeat_with(Vec::new).take(std::iter::repeat_with(Vec::new).take((((w/d).ceil()+1.0) * ((h/d).ceil() +1.0) * ((l/d).ceil() +1.0))as usize).collect())),
                    bags: RefCell::new(std::iter::repeat_with(Vec::new).take((((w/d).ceil()+1.0) * ((h/d).ceil() +1.0) * ((l/d).ceil() +1.0))as usize).collect()),
                    // bags: RefCell::new(std::iter::repeat_with(|| std::iter::repeat_with(|| Vec::new()).take(((l/d).ceil()+1.0) as usize).collect()).take(((h/d).ceil()+1.0) as usize).collect()),
                    rbags: RefCell::new(std::iter::repeat_with(Vec::new).take((((w/d).ceil()+1.0) * ((h/d).ceil() +1.0) * ((l/d).ceil() +1.0))as usize).collect()),
                    nagents: RefCell::new(0),
                    width: w,
                    height: h,
                    length: l,
                    discretization: d,
                    toroidal: t,
                    dh: ((h/d).ceil() as i32 +1),
                    dw: ((w/d).ceil() as i32 +1),
                    dz: ((l/d).ceil() as i32 +1),
                    density_estimation:0,
                    density_estimation_check:false
                }
            }

            /// Map coordinates of an object into matrix indexes
            ///
            /// # Arguments
            /// * `loc` - `Real3D` coordinates of the object
            fn discretize(&self, loc: &Real3D) -> Int3D {
                
                let x_floor = (loc.x/self.discretization).floor();
                let x_floor = x_floor as i32;

                let y_floor = (loc.y/self.discretization).floor();
                let y_floor = y_floor as i32;

                let z_floor = (loc.z/self.discretization).floor();
                let z_floor = z_floor as i32;
                Int3D {
                    x: x_floor,
                    y: y_floor,
                    z: z_floor
                }
            }

            /// Map matrix indexes into coordinates of an object
            ///
            /// # Arguments
            /// * `loc` - `Int3D` indexes of the object
            fn not_discretize(&self, loc: &Int3D) -> Real3D {
                let x_real = loc.x as f32 * self.discretization;
                let y_real = loc.y as f32 * self.discretization;
                let z_real = loc.z as f32 * self.discretization;

                Real3D {
                    x: x_real,
                    y: y_real,
                    z: z_real
                }
            }


            /// Return the set of objects within a certain distance
            ///
            /// # Arguments
            /// * `loc` - `Real3D` coordinates of the object
            /// * `dist` - Distance to look for objects
            
            /* 
            pub fn get_neighbors_within_distance(&self, loc: Real3D, dist: f32) -> Vec<O> {
                let mut neighbors: Vec<O>;

                if self.density_estimation_check {
                    neighbors = Vec::with_capacity(self.density_estimation*2);
                }else {neighbors = Vec::new();}

                if dist <= 0.0 {
                    return neighbors;
                }

                if dist <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (dist/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.height/self.discretization).ceil() as i32;

                let mut min_i = disc_loc.x - disc_dist;
                let mut max_i = disc_loc.x + disc_dist;
                let mut min_j = disc_loc.y - disc_dist;
                let mut max_j = disc_loc.y + disc_dist;

                if self.toroidal {
                    min_i = cmp::max(0, min_i);
                    max_i = cmp::min(max_i, max_x-1);
                    min_j = cmp::max(0, min_j);
                    max_j = cmp::min(max_j, max_y-1);
                }

                for i in min_i..max_i+1 {
                    for j in min_j..max_j+1 {
                        let bag_id = Int3D {
                            x: t_transform(i, max_x),
                            y: t_transform(j, max_y),
                        };

                        let check = check_circle(&bag_id, self.discretization, self.width, self.height, &loc, dist, self.toroidal);

                        let index = ((bag_id.x * self.dh) + bag_id.y) as usize;
                        let bags = self.rbags.borrow();

                        for elem in &bags[index]{
                            if (check == 0 && distance(&loc, &(elem.get_location()), self.width, self.height, self.length, self.toroidal) <= dist) || check == 1 {
                                neighbors.push(*elem);
                            }
                        }

                    }
                }
                neighbors
            }

            */

            /// Return the set of objects within a certain distance. No circle check.
            ///
            /// # Arguments
            /// * `loc` - `Real3D` coordinates of the object
            /// * `dist` - Distance to look for objects
            pub fn get_neighbors_within_relax_distance(&self, loc: Real3D, dist: f32) -> Vec<O> {
                let mut neighbors;

                if self.density_estimation_check {
                    neighbors = Vec::with_capacity(self.density_estimation*3);
                }else {
                    neighbors = Vec::new();
                }

                if dist <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (dist/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.height/self.discretization).ceil() as i32;
                let max_z =  (self.length/self.discretization).ceil() as i32;

                let mut min_i = disc_loc.x - disc_dist;
                let mut max_i = disc_loc.x + disc_dist;
                let mut min_j = disc_loc.y - disc_dist;
                let mut max_j = disc_loc.y + disc_dist;
                let mut min_k = disc_loc.z - disc_dist;
                let mut max_k = disc_loc.z + disc_dist;

                if self.toroidal {
                    min_i = cmp::max(0, min_i);
                    max_i = cmp::min(max_i, max_x-1);
                    min_j = cmp::max(0, min_j);
                    max_j = cmp::min(max_j, max_y-1);
                    min_k = cmp::max(0, min_k);
                    max_k = cmp::min(max_k, max_z-1);
                }

                for i in min_i..max_i+1 {
                    for j in min_j..max_j+1 {
                        for k in min_k..max_k+1 {
                            let bag_id = Int3D {
                                x: t_transform(i, max_x),
                                y: t_transform(j, max_y),
                                z: t_transform(k, max_z)
                            };

                            //TODO check index
                            //ind(i,j,k) = k + j*nz + i*ny*nz
                            // let index = ((bag_id.x * self.dh) + bag_id.y + bag_id.z) as usize;
                            // let index = ((bag_id.x * self.dw * self.dh) + bag_id.y*self.dh + bag_id.z) as usize;
                            //let index = ((bag_id.x * self.dw * self.dh) + bag_id.y*self.dh) as usize;
                            let index = ((bag_id.x * self.dw * self.dh) + bag_id.y*self.dh + bag_id.z) as usize;
                            let bags = self.rbags.borrow_mut();
                            for elem in &bags[index] {
                                neighbors.push(*elem);
                            }
                        }
                    }
                }
                neighbors
            }

            /// Return objects at a specific location
            ///
            /// # Arguments
            /// * `loc` - `Real3D` coordinates of the object
            pub fn get_objects(&self, loc: Real3D) -> Vec<O>{
                let bag = self.discretize(&loc);
                //TODO index
                // let index = ((bag.x * self.dh) + bag.y + bag.z) as usize;
                let index = ((bag.x * self.dw * self.dh) + bag.y*self.dh + bag.z) as usize;
                let rbags = self.rbags.borrow();
                rbags[index].clone()
            }

            /// Return objects at a specific location
            ///
            /// # Arguments
            /// * `loc` - `Real3D` coordinates of the object
            pub fn get_objects_unbuffered(&self, loc: Real3D) -> Vec<O>{
                let bag = self.discretize(&loc);
                // let index = ((bag.x * self.dh) + bag.y + bag.z) as usize;
                let index = ((bag.x * self.dw * self.dh) + bag.y*self.dh + bag.z) as usize;
                let bags = self.bags.borrow();
                bags[index].clone()
            }

            /// Iterate over the read state and apply the closure.
            ///
            /// # Arguments
            /// * `closure` - closure to apply to each element of the matrix
            pub fn iter_objects<F>(&self, closure: F)
            where
                F: Fn(
                        &Real3D, //location
                        &O, //value
                )
            {
                for i in 0 .. self.dw{
                    for j in 0 .. self.dh{
                        for k in 0 .. self.dz{
                            let index = ((i * self.dw * self.dh) + j*self.dh + k) as usize;
                            // let index = ((i * self.dh) + j + k) as usize;
                            let locs = &self.rbags.borrow()[index];
                            if !locs.is_empty() {
                                let real_pos = self.not_discretize(&Int3D {x: i, y: j, z: k});
                                for obj in locs{
                                    closure(&real_pos, obj);
                                }
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
            /// 
            /* 
            pub fn iter_objects_unbuffered<F>(&self, closure: F)
            where
                F: Fn(
                    &Real3D, //location
                    &O, //value
                )
            {
                for i in 0 .. self.dw{
                    for j in 0 .. self.dh{
                        let index = ((i * self.dh) + j) as usize;
                        let locs = &self.bags.borrow()[index];
                        if !locs.is_empty() {
                            let real_pos = self.not_discretize(&Int3D {x: i, y: j});
                            for obj in locs{
                                closure(&real_pos, obj);
                            }
                        }
                    }
                }
            }
            */

            /// Return all the empty bags from read state.
            pub fn get_empty_bags(&self) -> Vec<Real3D>{
                let mut empty_bags = Vec::new();
                for i in 0 ..  self.dw{
                    for j in 0 .. self.dh{
                        for k in 0 .. self.dz{
                            let index = ((i * self.dw * self.dh) + j*self.dh + k) as usize;
                            if self.rbags.borrow()[index].is_empty() {

                                empty_bags.push(self.not_discretize(&Int3D{x: i, y: j, z: k}));
                            }
                        }
                        
                    }
                }
                empty_bags
            }

            /// Return a random empty bag from read state. `None` if no bags are available.
            pub fn get_random_empty_bag(&self) -> Option<Real3D>{
                let empty_bags = self.get_empty_bags();
                if empty_bags.is_empty() {
                    return None;
                }
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..empty_bags.len());
                Some(empty_bags[index])
            }

            /// Return number of object at a specific location
            ///
            /// # Arguments
            /// * `loc` - `Real3D` coordinates of the object
            pub fn num_objects_at_location(&self, loc: Real3D) -> usize {
                let bag = self.discretize(&loc);
                let index = ((bag.x * self.dw * self.dh) + bag.y*self.dh + bag.z) as usize;
                let rbags = self.rbags.borrow();
                rbags[index].len()
            }

            /// Insert an object into a specific position
            ///
            /// # Arguments
            /// * `obj` - Object to insert
            /// * `loc` - `Real3D` coordinates of the object
            pub fn set_object_location(&self, object: O, loc: Real3D) {
                let bag = self.discretize(&loc);
                //println!("loc: {}, bag: {}", loc, bag);
                let index = ((bag.x * self.dw * self.dh) + bag.y*self.dh + bag.z) as usize;

                let mut bags = self.bags.borrow_mut();
                //println!("index: {}", index);
                
                bags[index].push(object);
                if !self.density_estimation_check{
                    *self.nagents.borrow_mut() += 1;
                }
            }

            /// Remove an object from a specific position.
            /// You have to use it to remove an object written/updated in this step.
            /// Double buffering swap the write and read state at the end of the step, so you have to call
            /// this function only if the object was written/set in this step.
            ///
            /// # Arguments
            /// * `object` - Object to remove
            /// * `loc` - `Real3D` coordinates of the object
            // TODO CHECK
            pub fn remove_object_location(&self, object: O, loc: Real3D) {
                let bag = self.discretize(&loc);
                let index = ((bag.x * self.dw * self.dh) + bag.y*self.dh + bag.z) as usize;
                let mut bags = self.bags.borrow_mut();
                if !bags[index].is_empty() {
                    let before = bags[index].len();
                    bags[index].retain(|&x| x != object);
                    let after = bags[index].len();

                    if !self.density_estimation_check{
                        *self.nagents.borrow_mut() -= before - after;
                    }
                }
            }
        }

        impl<'a, O: Location3D<Real3D> + Clone + Hash + Eq + Copy + Display> Field for Field3D<O>{
            fn update(&mut self){}

            /// Swap read and write buffer
            fn lazy_update(&mut self){
                unsafe {
                    std::ptr::swap(
                        self.bags.as_ptr(),
                        self.rbags.as_ptr(),
                    )
                }
                if !self.density_estimation_check{
                    self.density_estimation =
                    ((*self.nagents.borrow_mut())as usize)/((self.dw * self.dh * self.dz) as usize);
                    self.density_estimation_check = true;
                    self.bags = RefCell::new(std::iter::repeat_with(Vec::new).take((((self.width/self.discretization).ceil()+1.0) * ((self.height/self.discretization).ceil() +1.0) * ((self.length/self.discretization).ceil() +1.0))as usize).collect());
                }
                else {
                    let mut bags =self.bags.borrow_mut();
                    for b in 0..bags.len(){
                        bags[b].clear();
                    }
                }
            }
        }
    }
}

fn t_transform(n: i32, size: i32) -> i32 {
    if n >= 0 {
        n % size
    } else {
        (n % size) + size
    }
}
/* 
fn check_circle(
    bag: &Int3D,
    discretization: f32,
    width: f32,
    height: f32,
    length: f32,
    loc: &Real3D,
    dis: f32,
    tor: bool,
) -> i8 {
    let nw = Real3D {
        x: (bag.x as f32) * discretization,
        y: (bag.y as f32) * discretization,
        z: (bag.z as f32) * discretization,
    };
    let ne = Real3D {
        x: nw.x,
        y: (nw.y + discretization).min(height),
    };
    let sw = Real3D {
        x: (nw.x + discretization).min(width),
        y: nw.y,
    };
    let se = Real3D { x: sw.x, y: ne.y };

    if distance(&nw, loc, width, height, length, tor) <= dis
        && distance(&ne, loc, width, height, length, tor) <= dis
        && distance(&sw, loc, width, height, length, tor) <= dis
        && distance(&se, loc, width, height, length, tor) <= dis
    {
        1
    } else if distance(&nw, loc, width, height, length, tor) > dis
        && distance(&ne, loc, width, height, length, tor) > dis
        && distance(&sw, loc, width, height, length, tor) > dis
        && distance(&se, loc, width, height, length, tor) > dis
    {
        -1
    } else {
        0
    }
}
*/
//TODO sqrt o radice cubica?
fn distance(loc1: &Real3D, loc2: &Real3D, dim1: f32, dim2: f32, dim3: f32, tor: bool) -> f32 {
    let dx;
    let dy;
    let dz;

    if tor {
        dx = toroidal_distance(loc1.x, loc2.x, dim1);
        dy = toroidal_distance(loc1.y, loc2.y, dim2);
        dz = toroidal_distance(loc1.z, loc2.z, dim3);
    } else {
        dx = loc1.x - loc2.x;
        dy = loc1.y - loc2.y;
        dz = loc1.z - loc2.z;
    }
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn toroidal_distance(val1: f32, val2: f32, dim: f32) -> f32 {
    if (val1 - val2).abs() <= dim / 2.0 {
        return val1 - val2;
    }

    let d = toroidal_transform(val1, dim) - toroidal_transform(val2, dim);

    if d * 2.0 > dim {
        d - dim
    } else if d * 2.0 < -dim {
        d + dim
    } else {
        d
    }
}

pub fn toroidal_transform(val: f32, dim: f32) -> f32 {
    if val >= 0.0 && val < dim {
        val
    } else {
        let mut val = val % dim;
        if val < 0.0 {
            val += dim;
        }
        val
    }
}
