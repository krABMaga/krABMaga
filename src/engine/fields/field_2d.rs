use crate::engine::{
    fields::field::Field,
    location::{Int2D, Location2D, Real2D},
};

use cfg_if::cfg_if;
use core::fmt::Display;
use std::cmp;
use std::hash::Hash;

cfg_if! {
    if #[cfg(any(feature = "visualization", feature = "visualization_wasm", feature = "parallel"))] {
        use crate::utils::dbdashmap::DBDashMap;
    } else {
        use std::cell::RefCell;
    }
}

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        pub struct Field2D<O: Location2D<Real2D> + Clone + Hash + Eq + Copy + Display> {
            pub findex: DBDashMap<O, Int2D>,
            pub fbag: DBDashMap<Int2D, Vec<O>>,
            pub floc: DBDashMap<O, Real2D>,
            pub width: f32,
            pub heigth: f32,
            pub discretization: f32,
            pub toroidal: bool,
        }

        //field 2d
        impl<O: Location2D<Real2D> + Clone + Hash + Eq + Copy + Display> Field2D<O> {
            pub fn new(w: f32, h: f32, d: f32, t: bool) -> Field2D<O> {
                Field2D {
                    findex: DBDashMap::new(),
                    fbag: DBDashMap::new(),
                    floc: DBDashMap::new(),
                    width: w,
                    heigth: h,
                    discretization: d,
                    toroidal: t,
                }
            }

            fn discretize(&self, loc: &Real2D) -> Int2D {
                let x_floor = (loc.x/self.discretization).floor();
                let x_floor = x_floor as i32;

                let y_floor = (loc.y/self.discretization).floor();
                let y_floor = y_floor as i32;

                Int2D {
                    x: x_floor,
                    y: y_floor,
                }
            }

            pub fn get_neighbors_within_distance(&self, loc: Real2D, dist: f32) -> Vec<O> {

                let density = ((self.width * self.heigth) as usize)/(self.findex.r_len());
                let sdist = (dist * dist) as usize;
                let mut neighbors: Vec<O> = Vec::with_capacity(density as usize * sdist);

                if dist <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (dist/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.heigth/self.discretization).ceil() as i32;

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
                        let bag_id = Int2D {
                            x: t_transform(i, max_x),
                            y: t_transform(j, max_y),
                        };
                        let check = check_circle(&bag_id, self.discretization, self.width, self.heigth, &loc, dist, self.toroidal);
                        let vector =  match self.fbag.get_read(&bag_id) {
                            Some(i) => i,
                            None => continue,
                        };

                        for elem in vector{
                            if (check == 0 &&
                                distance(&loc, &(elem.get_location()), self.width, self.heigth, self.toroidal) <= dist) ||
                                check == 1
                            {
                                neighbors.push(*elem);
                            }
                        }
                    }
                }
                neighbors
            }

            pub fn get_neighbors_within_relax_distance(&self, loc: Real2D, dist: f32) -> Vec<O> {

                let density = ((self.width * self.heigth) as usize)/(self.findex.r_len());
                let sdist = (dist * dist) as usize;
                let mut neighbors: Vec<O> = Vec::with_capacity(density as usize * sdist);

                if dist <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (dist/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.heigth/self.discretization).ceil() as i32;

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
                        let bag_id = Int2D {
                            x: t_transform(i, max_x),
                            y: t_transform(j, max_y),
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
            pub fn get_objects(&self, loc: Real2D) -> Vec<&O> {
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

            // TODO
            // take a location and return the corresponding objects on that location from the write state
            // pub fn get_objects_unbuffered(&self, loc: Real2D) -> Vec<&O> {
            // }

            // take an object and return the corresponding location
            pub fn get_location(&self, object: O) -> Option<&Real2D> {
                self.floc.get_read(&object)
            }

            // take an object and return the corresponding location from the write state
            pub fn get_location_unbuffered(&self, object: O) -> Option<Real2D> {
                let mut loc = self.floc.get_write(&object).unwrap();
                Some(*loc.value_mut())
            }

            // take an object and check if it is in the field
            // if so return the object from the write bags
            // mainly used for visualization
            pub fn get_unbuffered(&self, object: &O) -> Option<O> {

                match self.floc.get_write(object){
                    Some(loc) =>{
                        let real_loc = self.discretize(&loc);
                        for obj in self.fbag.get_write(&real_loc).unwrap().value_mut(){
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
            pub fn num_objects_at_location(&self, loc: Real2D) -> usize {
                let bag = self.discretize(&loc);
                match self.fbag.get_read(&bag){
                    Some(v) => {
                        v.len()
                    }
                    None => 0
                }
            }

            // put the object in that location
            pub fn set_object_location(&self, object: O, loc: Real2D) {
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

        impl<O: Location2D<Real2D> + Clone + Hash + Eq + Copy + Display> Field for Field2D<O>{
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
        pub struct Field2D<O: Location2D<Real2D> + Clone + Hash + Eq + Copy + Display> {
            pub bags: RefCell<Vec<Vec<O>>>,
            pub rbags: RefCell<Vec<Vec<O>>>,
            pub nagents: RefCell<usize>,
            pub width: f32,
            pub heigth: f32,
            pub discretization: f32,
            pub toroidal: bool,
            pub dh: i32,
            pub dw: i32,
            pub density_estimation:usize,
            pub density_estimation_check:bool,
        }
        impl<O: Location2D<Real2D> + Clone + Hash + Eq + Copy + Display> Field2D<O>  {

            pub fn new(w: f32, h: f32, d: f32, t: bool) -> Field2D<O> {
                Field2D {
                    bags: RefCell::new(std::iter::repeat_with(Vec::new).take((((w/d).ceil()+1.0) * ((h/d).ceil() +1.0))as usize).collect()),
                    rbags: RefCell::new(std::iter::repeat_with(Vec::new).take((((w/d).ceil()+1.0) * ((h/d).ceil() +1.0))as usize).collect()),
                    nagents: RefCell::new(0),
                    width: w,
                    heigth: h,
                    discretization: d,
                    toroidal: t,
                    dh: ((h/d).ceil() as i32 +1),
                    dw: ((w/d).ceil() as i32 +1),
                    density_estimation:0,
                    density_estimation_check:false
                }
            }

            fn discretize(&self, loc: &Real2D) -> Int2D {
                let x_floor = (loc.x/self.discretization).floor();
                let x_floor = x_floor as i32;

                let y_floor = (loc.y/self.discretization).floor();
                let y_floor = y_floor as i32;

                Int2D {
                    x: x_floor,
                    y: y_floor,
                }
            }

            pub fn get_neighbors_within_distance(&self, loc: Real2D, dist: f32) -> Vec<O> {
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
                let max_y =  (self.heigth/self.discretization).ceil() as i32;

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
                        let bag_id = Int2D {
                            x: t_transform(i, max_x),
                            y: t_transform(j, max_y),
                        };

                        let check = check_circle(&bag_id, self.discretization, self.width, self.heigth, &loc, dist, self.toroidal);

                        let index = ((bag_id.x * self.dh) + bag_id.y) as usize;
                        let bags = self.rbags.borrow();

                        for elem in &bags[index]{
                            if (check == 0 && distance(&loc, &(elem.get_location()), self.width, self.heigth, self.toroidal) <= dist) || check == 1 {
                                neighbors.push(*elem);
                            }
                        }

                    }
                }
                neighbors
            }

            pub fn get_neighbors_within_relax_distance(&self, loc: Real2D, dist: f32) -> Vec<O> {
                let mut neighbors;

                if self.density_estimation_check {
                    neighbors = Vec::with_capacity(self.density_estimation*2);
                }else {
                    neighbors = Vec::new();
                }

                if dist <= 0.0 {
                    return neighbors;
                }

                let disc_dist = (dist/self.discretization).floor() as i32;
                let disc_loc = self.discretize(&loc);
                let max_x = (self.width/self.discretization).ceil() as i32;
                let max_y =  (self.heigth/self.discretization).ceil() as i32;

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
                        let bag_id = Int2D {
                            x: t_transform(i, max_x),
                            y: t_transform(j, max_y),
                        };
                        let index = ((bag_id.x * self.dh) + bag_id.y) as usize;
                        let bags = self.rbags.borrow_mut();
                        for elem in &bags[index] {
                            neighbors.push(*elem);
                        }
                    }
                }
                neighbors
            }

            pub fn get_objects(&self, loc: Real2D) -> Vec<O>{
                let bag = self.discretize(&loc);
                let index = ((bag.x * self.dh) + bag.y) as usize;
                let rbags = self.rbags.borrow();
                rbags[index].clone()
            }

            pub fn num_objects_at_location(&self, loc: Real2D) -> usize {
                let bag = self.discretize(&loc);
                let index = ((bag.x * self.dh) + bag.y) as usize;
                let rbags = self.rbags.borrow();
                rbags[index].len()
            }

            pub fn set_object_location(&self, object: O, loc: Real2D) {
                let bag = self.discretize(&loc);
                let index = ((bag.x * self.dh) + bag.y) as usize;
                let mut bags = self.bags.borrow_mut();
                bags[index].push(object);
                if !self.density_estimation_check{
                    *self.nagents.borrow_mut() += 1;
                }
            }
        }

        impl<'a, O: Location2D<Real2D> + Clone + Hash + Eq + Copy + Display> Field for Field2D<O>{
            fn update(&mut self){}

            fn lazy_update(&mut self){
                unsafe {
                    std::ptr::swap(
                        self.bags.as_ptr(),
                        self.rbags.as_ptr(),
                    )
                }
                if !self.density_estimation_check{
                    self.density_estimation =
                    ((*self.nagents.borrow_mut())as usize)/((self.dw * self.dh) as usize);
                    self.density_estimation_check = true;
                    self.bags =  RefCell::new(std::iter::repeat_with(|| Vec::with_capacity(self.density_estimation)).take((self.dw * self.dh) as usize).collect());
                }else{
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

fn check_circle(
    bag: &Int2D,
    discretization: f32,
    width: f32,
    heigth: f32,
    loc: &Real2D,
    dis: f32,
    tor: bool,
) -> i8 {
    let nw = Real2D {
        x: (bag.x as f32) * discretization,
        y: (bag.y as f32) * discretization,
    };
    let ne = Real2D {
        x: nw.x,
        y: (nw.y + discretization).min(heigth),
    };
    let sw = Real2D {
        x: (nw.x + discretization).min(width),
        y: nw.y,
    };
    let se = Real2D { x: sw.x, y: ne.y };

    if distance(&nw, loc, width, heigth, tor) <= dis
        && distance(&ne, loc, width, heigth, tor) <= dis
        && distance(&sw, loc, width, heigth, tor) <= dis
        && distance(&se, loc, width, heigth, tor) <= dis
    {
        1
    } else if distance(&nw, loc, width, heigth, tor) > dis
        && distance(&ne, loc, width, heigth, tor) > dis
        && distance(&sw, loc, width, heigth, tor) > dis
        && distance(&se, loc, width, heigth, tor) > dis
    {
        -1
    } else {
        0
    }
}

fn distance(loc1: &Real2D, loc2: &Real2D, dim1: f32, dim2: f32, tor: bool) -> f32 {
    let dx;
    let dy;

    if tor {
        dx = toroidal_distance(loc1.x, loc2.x, dim1);
        dy = toroidal_distance(loc1.y, loc2.y, dim2);
    } else {
        dx = loc1.x - loc2.x;
        dy = loc1.y - loc2.y;
    }
    (dx * dx + dy * dy).sqrt()
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
