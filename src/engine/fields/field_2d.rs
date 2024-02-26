use core::fmt::Display;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bevy::prelude::{Component, Entity, Query};

use crate::engine::components::double_buffer::DBWrite;
use crate::engine::components::position::Real2DTranslation;
use crate::engine::location::{Int2D, Real2D};

/// A trait to request implementation of the two basic function that must be implemented
pub trait Location2D<Real2D> {
    fn get_location(self) -> Real2D;
    fn set_location(&mut self, loc: Real2D);
}

/// Immutably fetch DBWrite transforms to get the latest value, by assuming this system runs after any modelist system.
/// We use DBWrite instead of DBRead to get the updated values and to run this in parallel with the DB update system.
/// TODO should this run after the doublebuffer updates to use DBRead here, as to make it clearer/safer?
pub fn update_field(
    mut field_query: Query<&mut Field2D<Entity>>,
    xform_query: Query<(Entity, &DBWrite<Real2DTranslation>)>,
) {
    if let Ok(mut field) = field_query.get_single_mut() {
        field.clear();
        for (entity, xform) in &xform_query {
            let xform = xform.0;
            field.set_object_location(entity, xform.0);
        }
    }
}

// TODO compare this 2022 impl with the current one
///  Sparse matrix structure modelling agent interactions on a 2D real space with coordinates represented by 2D f32 tuples
#[derive(Component)]
pub struct Field2D<O: Copy + Eq + Hash> {
    /// Matrix to write data. Vector of vectors that have a generic Object O inside
    pub findex: HashMap<O, Int2D>,
    pub fbag: HashMap<Int2D, Vec<O>>,
    pub floc: HashMap<O, Real2D>,
    /// First dimension of the field
    pub width: f32,
    /// Second dimension of the field
    pub height: f32,
    /// Value to discretize `Real2D` positions to our Matrix
    pub discretization: f32,
    /// `true` if you want a Toroidal field, `false` otherwise
    pub toroidal: bool,
}

impl<O: Hash + Eq + Copy> Field2D<O> {
    /// Create a new `Field2D`
    ///
    /// # Arguments
    /// * `w` - Width, first dimension of the field
    /// * `h` - Height, second dimension of the field
    /// * `d` - Value to discretize `Real2D` positions to our Matrix
    /// * `t` - `true` if you want a Toroidal field, `false` otherwise
    pub fn new(w: f32, h: f32, d: f32, t: bool) -> Field2D<O> {
        Field2D {
            findex: HashMap::default(),
            fbag: HashMap::default(),
            floc: HashMap::default(),
            width: w,
            height: h,
            discretization: d,
            toroidal: t,
        }
    }

    pub fn clear(&mut self) {
        self.findex.clear();
        self.fbag.clear();
        self.floc.clear();
    }

    pub fn get_location(&self, object: O) -> Option<&Real2D> {
        self.floc.get(&object)
    }

    /// Map coordinates of an object into matrix indexes
    ///
    /// # Arguments
    /// * `loc` - `Real2D` coordinates of the object
    fn discretize(&self, loc: &Real2D) -> Int2D {
        let x_floor = (loc.x / self.discretization).floor();
        let x_floor = x_floor as i32;

        let y_floor = (loc.y / self.discretization).floor();
        let y_floor = y_floor as i32;

        Int2D {
            x: x_floor,
            y: y_floor,
        }
    }

    /// Map matrix indexes into coordinates of an object
    ///
    /// # Arguments
    /// * `loc` - `Int2D` indexes of the object
    fn not_discretize(&self, loc: &Int2D) -> Real2D {
        let x_real = loc.x as f32 * self.discretization;
        let y_real = loc.y as f32 * self.discretization;

        Real2D {
            x: x_real,
            y: y_real,
        }
    }

    /// Return the set of objects within a certain distance.
    ///
    /// # Arguments
    /// * `loc` - `Real2D` coordinates of the object
    /// * `dist` - Distance to look for objects
    ///
    /// # Example
    /// ```
    /// struct Object {
    ///    id: u32
    /// }
    ///
    /// let DISCRETIZATION = 0.5;
    /// let TOROIDAL = true;
    /// let mut field = Field2D::new(10.0,  10.0, DISCRETIZATION, TOROIDAL);
    /// field.set_object_location(&Object{id: 0}, Real2D {x: 0.0, y: 0.0} );
    /// field.set_object_location(&Object{id: 1}, Real2D {x: 3.0, y: 3.0} );
    ///
    /// field.lazy_update();
    ///
    /// let objects = field.get_objects_within_distance(&Real2D {x: 0.0, y: 0.0}, 2.0);
    /// assert_eq!(objects.len(), 1);
    ///
    /// let objects = field.get_objects_within_distance(&Real2D {x: 0.0, y: 0.0}, 5.0);
    /// assert_eq!(objects.len(), 2);
    ///
    /// let objects = field.get_objects_within_distance(&Real2D {x: 6.0, y: 6.0}, 1.0);
    /// assert_eq!(objects.len(), 0);
    ///
    /// ```
    pub fn get_neighbors_within_distance(&self, loc: Real2D, dist: f32) -> HashSet<O> {
        let density = ((self.width * self.height) as usize) / (self.findex.len());
        let sdist = (dist * dist) as usize;
        let mut neighbors: HashSet<O> = HashSet::with_capacity(density * sdist);

        if dist <= 0.0 {
            return neighbors;
        }

        let disc_dist = (dist / self.discretization).floor() as i32;
        let disc_loc = self.discretize(&loc);
        let max_x = (self.width / self.discretization).ceil() as i32;
        let max_y = (self.height / self.discretization).ceil() as i32;

        let mut min_i = disc_loc.x - disc_dist;
        let mut max_i = disc_loc.x + disc_dist;
        let mut min_j = disc_loc.y - disc_dist;
        let mut max_j = disc_loc.y + disc_dist;

        if self.toroidal {
            min_i = cmp::max(0, min_i);
            max_i = cmp::min(max_i, max_x - 1);
            min_j = cmp::max(0, min_j);
            max_j = cmp::min(max_j, max_y - 1);
        }

        for i in min_i..max_i + 1 {
            for j in min_j..max_j + 1 {
                let bag_id = Int2D {
                    x: t_transform(i, max_x),
                    y: t_transform(j, max_y),
                };
                let check = check_circle(
                    &bag_id,
                    self.discretization,
                    self.width,
                    self.height,
                    &loc,
                    dist,
                    self.toroidal,
                );
                let vector = match self.fbag.get(&bag_id) {
                    Some(i) => i,
                    None => continue,
                };

                for elem in vector {
                    if (check == 0
                        && distance(
                            &loc,
                            self.get_location(*elem).unwrap(),
                            self.width,
                            self.height,
                            self.toroidal,
                        ) <= dist)
                        || check == 1
                    {
                        neighbors.insert(*elem);
                    }
                }
            }
        }
        neighbors
    }

    /// Return the set of objects within a certain distance. No circle check.
    ///
    /// # Arguments
    /// * `loc` - `Real2D` coordinates of the object
    /// * `dist` - Distance to look for objects
    ///
    /// # Example
    /// ```
    /// struct Object {
    ///   id: u32
    /// }
    ///
    /// let DISCRETIZATION = 0.5;
    /// let TOROIDAL = true;
    /// let mut field = Field2D::new(10.0,  10.0, DISCRETIZATION, TOROIDAL);
    /// field.set_object_location(&Object{id: 0}, Real2D {x: 0.0, y: 0.0} );
    /// field.set_object_location(&Object{id: 1}, Real2D {x: 3.0, y: 3.0} );
    ///
    /// field.lazy_update();
    ///
    /// let objects = field.get_objects_within_relax_distance(&Real2D {x: 0.0, y: 0.0}, 2.0);
    /// assert_eq!(objects.len(), 1);
    ///
    /// let objects = field.get_objects_within_relax_distance(&Real2D {x: 0.0, y: 0.0}, 5.0);
    /// assert_eq!(objects.len(), 2);
    ///
    /// let objects = field.get_objects_within_relax_distance(&Real2D {x: 6.0, y: 6.0}, 1.0);
    /// assert_eq!(objects.len(), 0);
    ///
    /// ```
    pub fn get_neighbors_within_relax_distance(&self, loc: Real2D, dist: f32) -> Vec<O> {
        let density = ((self.width * self.height) as usize) / (self.findex.len());
        let sdist = (dist * dist) as usize;
        let mut neighbors: Vec<O> = Vec::with_capacity(density * sdist);

        if dist <= 0.0 {
            return neighbors;
        }

        let disc_dist = (dist / self.discretization).floor() as i32;
        let disc_loc = self.discretize(&loc);
        let max_x = (self.width / self.discretization).ceil() as i32;
        let max_y = (self.height / self.discretization).ceil() as i32;

        let mut min_i = disc_loc.x - disc_dist;
        let mut max_i = disc_loc.x + disc_dist;
        let mut min_j = disc_loc.y - disc_dist;
        let mut max_j = disc_loc.y + disc_dist;

        if self.toroidal {
            min_i = cmp::max(0, min_i);
            max_i = cmp::min(max_i, max_x - 1);
            min_j = cmp::max(0, min_j);
            max_j = cmp::min(max_j, max_y - 1);
        }

        for i in min_i..max_i + 1 {
            for j in min_j..max_j + 1 {
                let bag_id = Int2D {
                    x: t_transform(i, max_x),
                    y: t_transform(j, max_y),
                };
                let vector = match self.fbag.get(&bag_id) {
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

    /// Return objects at a specific location
    ///
    /// # Arguments
    /// * `loc` - `Real2D` coordinates of the object
    ///
    /// # Example
    /// ```
    /// struct Object {
    ///  id: u32
    /// }
    ///
    /// let DISCRETIZATION = 0.5;
    /// let TOROIDAL = true;
    /// let mut field = Field2D::new(10.0,  10.0, DISCRETIZATION, TOROIDAL);
    ///
    /// field.set_object_location(&Object{id: 0}, Real2D {x: 5.0, y: 5.0} );
    /// field.set_object_location(&Object{id: 1}, Real2D {x: 5.0, y: 5.0} );
    ///
    /// let none = field.get_objects(&Real2D {x: 5.0, y: 5.0});
    /// assert_eq!(none.len(), 0);
    ///
    /// field.lazy_update();
    /// let objects = field.get_objects(&Real2D {x: 5.0, y: 5.0});
    /// assert_eq!(objects.len(), 2);
    ///
    /// ```
    pub fn get_objects(&self, loc: Real2D) -> Vec<&O> {
        let bag = self.discretize(&loc);
        let mut result = Vec::new();

        match self.fbag.get(&bag) {
            Some(v) => {
                for el in v {
                    result.push(el);
                }
            }
            None => (),
        }
        result
    }

    /// Return number of object at a specific location
    ///
    /// # Arguments
    /// * `loc` - `Real2D` coordinates of the location to check
    ///
    /// # Example
    /// ```
    /// struct Object {
    ///     id: u32
    /// }
    ///
    /// let DISCRETIZATION = 0.5;
    /// let TOROIDAL = true;
    /// let mut field = Field2D::new(10.0,  10.0, DISCRETIZATION, TOROIDAL);
    ///
    /// field.set_object_location(&Object{id: 0}, Real2D {x: 5.0, y: 5.0} );
    /// field.set_object_location(&Object{id: 1}, Real2D {x: 1.5, y: 1.5} );
    /// field.set_object_location(&Object{id: 2}, Real2D {x: 4.0, y: 4.0} );
    ///
    /// field.lazy_update();
    ///
    /// let one = field.get_number_of_objects_at_location(&Real2D {x: 5.0, y: 5.0});
    /// assert_eq!(one, 1);
    /// let two = field.get_number_of_objects_at_location(&Real2D {x: 1.5, y: 1.5});
    /// assert_eq!(two, 2);
    /// let zero = field.get_number_of_objects_at_location(&Real2D {x: 8.0, y: 8.0});
    /// assert_eq!(zero, 0);
    /// ```
    ///
    pub fn num_objects_at_location(&self, loc: Real2D) -> usize {
        let bag = self.discretize(&loc);
        match self.fbag.get(&bag) {
            Some(v) => v.len(),
            None => 0,
        }
    }

    /// Insert an object into a specific position
    ///
    /// # Arguments
    /// * `obj` - Object to insert
    /// * `loc` - `Real2D` coordinates where to insert the object
    ///
    /// # Example
    /// ```
    /// struct Object {
    ///    id: u32
    /// }
    ///
    /// let DISCRETIZATION = 0.5;
    /// let TOROIDAL = true;
    /// let mut field = Field2D::new(10.0,  10.0, DISCRETIZATION, TOROIDAL);
    ///
    /// field.set_object_location(&Object{id: 0}, Real2D {x: 5.0, y: 5.0} );
    ///
    /// let obj = field.get_objects_unbuffered(&Real2D {x: 5.0, y: 5.0});
    /// assert_eq!(obj.len(), 1);
    /// assert_eq!(obj[0].id, 0);
    ///
    /// field.lazy_update();
    /// let obj = field.get_objects(&Real2D {x: 5.0, y: 5.0});
    /// assert_eq!(obj.len(), 1);
    /// assert_eq!(obj[0].id, 0);
    ///
    /// ```
    pub fn set_object_location(&mut self, object: O, loc: Real2D) {
        let bag = self.discretize(&loc);
        self.floc.insert(object, loc);
        self.findex.insert(object, bag);
        match self.fbag.get_mut(&bag) {
            Some(v) => {
                v.push(object);
            }
            None => {
                let mut v = Vec::new();
                v.push(object);
                self.fbag.insert(bag, v);
            }
        };
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
    height: f32,
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
        y: (nw.y + discretization).min(height),
    };
    let sw = Real2D {
        x: (nw.x + discretization).min(width),
        y: nw.y,
    };
    let se = Real2D { x: sw.x, y: ne.y };

    if distance(&nw, loc, width, height, tor) <= dis
        && distance(&ne, loc, width, height, tor) <= dis
        && distance(&sw, loc, width, height, tor) <= dis
        && distance(&se, loc, width, height, tor) <= dis
    {
        1
    } else if distance(&nw, loc, width, height, tor) > dis
        && distance(&ne, loc, width, height, tor) > dis
        && distance(&sw, loc, width, height, tor) > dis
        && distance(&se, loc, width, height, tor) > dis
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
