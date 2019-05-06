use crate::location::Real2D;
use std::hash::Hash;
use crate::location::Int2D;
use crate::location::Location2D;
use std::collections::HashMap;
use std::cmp;

#[derive(Clone)]
pub struct Field2D<A: Location2D + Clone + Hash + Eq> {
    pub vec : Vec<A>,
    pub findex: HashMap<A, Int2D>,
    pub fbag: HashMap<Int2D, HashMap<A, Real2D>>,
    pub fpos: HashMap<A, Real2D>,
    pub width: f64,
    pub heigth: f64,
    pub discretization: f64,
    pub toroidal: bool,
}

impl<A: Location2D + Clone + Hash + Eq> Field2D<A> {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> Field2D<A> {
        Field2D {
            vec: Vec::new(),
            findex: HashMap::new(),
            fbag: HashMap::new(),
            fpos: HashMap::new(),
            width: w,
            heigth: h,
            discretization: d,
            toroidal: t,
        }
    }

    pub fn set_object_location(&mut self, object: A, pos: Real2D) {
        let bag = self.discretize(pos.clone());

            match self.fpos.get(&object) {
                Some(x) => {
                    if *x == pos {return}
                    else {
                        match self.findex.get(&object) {
                            Some(x) => {
                                if *x == bag {
                                    self.fpos.insert(object.clone(), pos.clone());
                                    return;
                                } else {
                                    self.findex.insert(object.clone(), bag.clone());
                                    self.fpos.insert(object.clone(), pos.clone());

                                }
                            },
                            None => {
                                panic!("Errore controllo esistenza")
                            }
                        }
                    }
                },
                None => {
                    self.findex.insert(object.clone(), bag.clone());
                    self.fpos.insert(object.clone(), pos.clone());
                }
            }

        //self.vec.push(object);
        //print!("lunghezza vett {}", self.vec.len())
    }

    pub fn get_neighbors_within_distance(&self, _object: &A, pos: Real2D, dist: f64) -> Vec<A> {
        let mut tor: Vec<A> = Vec::new();

        if dist <= 0.0 {
            return tor;
        }

        let disc_dist = (dist/self.discretization).floor() as i64;
        let disc_pos = self.discretize(pos);
        let max_x = (self.width/self.discretization).ceil() as i64;
        let max_y =  (self.heigth/self.discretization).ceil() as i64;

        let mut min_i = disc_pos.x - disc_dist;
        let mut max_i = disc_pos.x + disc_dist;
        let mut min_j = disc_pos.y - disc_dist;
        let mut max_j = disc_pos.y + disc_dist;

        if self.toroidal {
            min_i = cmp::max(0, min_i);
            max_i = cmp::min(max_i, max_x-1);
            min_j = cmp::max(0, min_j);
            max_j = cmp::min(max_j, max_y-1);
        }

        for i in min_i..max_i {
            for j in min_j..max_j {
                let bag_id = Int2D {
                    x: t_transform(i, max_x),
                    y: t_transform(j, max_y),
                };

                

            }
        }


        //let x = (self.vec.len()/100)*10;
        for y in 1..self.vec.len() {
            tor.push(self.vec[y].clone())
        }
        //let x = self.vec.clone();
        //x
        // for i in 0..3 {
        //     vec2.push(x[i]);
        // }
        tor
    }

    fn get_objects_at_location(){

    }
    fn num_objects_at_location() {

    }
    fn get_object_location() {

    }

    fn discretize(&self, pos: Real2D) -> Int2D {
        let x_floor = (pos.x/self.discretization).floor();
        let x_floor = x_floor as i64;
        let y_floor = (pos.y/self.discretization).floor();
        let y_floor = y_floor as i64;

        Int2D {
            x: x_floor,
            y: y_floor,
        }
    }

}

fn t_transform(n: i64, size: i64) -> i64 {
    if n >= 0 {
        return n%size;
    } else {
        return (n%size) + size;
    }
}
