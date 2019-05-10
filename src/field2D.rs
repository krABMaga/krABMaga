use std::fmt::Display;
use crate::location::Real2D;
use std::hash::Hash;
use crate::location::Int2D;
use crate::location::Location2D;
use std::collections::HashMap;
use std::cmp;

#[derive(Clone)]
pub struct Field2D<A: Location2D + Clone + Hash + Eq + Display + Copy> {
    pub vec : Vec<A>,
    pub findex: HashMap<A, Int2D>,
    pub fbag: HashMap<Int2D, Vec<A>>,
    pub fpos: HashMap<A, Real2D>,
    pub width: f64,
    pub heigth: f64,
    pub discretization: f64,
    pub toroidal: bool,
}

impl<A: Location2D + Clone + Hash + Eq + Display + Copy> Field2D<A> {
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
        let bag = self.discretize(&pos);
        //println!("{} {}", bag.x, bag.y);

        // for (key, val) in self.fpos.iter() {
        //     println!("key: {} val: {}", key, val);
        // }


        match self.fpos.get(&object) {
            Some(x) => {
                if *x == pos {
                    //println!("posizione 1");
                    return;
                }
                else {
                    //println!("posizione 2");

                    match self.findex.get(&object) {
                        Some(x) => {
                            if *x == bag {
                                //println!("posizione 3");

                                //clone entrambe
                                self.fpos.insert(object, pos);
                                return;
                            } else {
                                //println!("posizione 4");

                                let oldbag = self.findex.get(&object);
                                let oldbag = match oldbag {
                                    Some(i) => i,
                                    None => panic!("error oldbag"),
                                };
                                //let mut index = 0;
                                //
                                //  CHANGE to vec
                                //+
                                println!("{}", object);
                                self.fbag.get_mut(oldbag).unwrap().retain(|x| *x == object);

                                // let mut vector =  match self.fbag.get_mut(oldbag) {
                                //     Some(i) => {
                                //         for elem in i {
                                //             // elem
                                //             if *elem == object {
                                //                 i.remove(index);
                                //                 break;
                                //             }
                                //             index += 1;
                                //         }
                                //
                                //     }
                                //     None => panic!("error vector from oldbag"),
                                // };

                                self.findex.insert(object, bag);
                                self.fpos.insert(object, pos);

                                if !self.fbag.contains_key(&bag) {
                                    let mut vec: Vec<A> = Vec::new();
                                    vec.push(object);
                                    self.fbag.insert(bag, vec);
                                } else {
                                    let mut vec = match self.fbag.get(&bag) {
                                        Some(i) => i.to_vec(),
                                        None => panic!("error vector from bag"),
                                    };
                                    vec.push(object.clone());
                                }

                            }
                        },
                        None => {
                            panic!("Errore controllo esistenza")
                        }
                    }
                }
            },
            None => {
                // println!("posizione 5");

                self.findex.insert(object, bag);
                self.fpos.insert(object, pos);

                if !self.fbag.contains_key(&bag) {
                    // println!("posizione 6");

                    let mut vec: Vec<A> = Vec::new();
                    //clone
                    vec.push(object);
                    //clone bag
                    self.fbag.insert(bag, vec);
                } else {
                    // println!("posizione 7");

                    // let mut vec = match self.fbag.get(&bag) {
                    //     Some(v) => *v.push(object.clone()),
                    //     None => panic!("error vector from bag 2"),
                    // };
                    //clone object
                    self.fbag.get_mut(&bag).unwrap().push(object);
                }
                    //println!("{} {}", bag.x, bag.y);
            }
        }
    //self.vec.push(object);
    //print!("lunghezza vett {}", self.vec.len())
    }

    pub fn get_neighbors_within_distance(&self, pos: Real2D, dist: f64) -> Vec<A> {
        // for (key, value) in self.fbag.clone() {
        //     println!("chiave {} {}", key.x, key.y);
        //     for elem in value {
        //         println!("{}", elem);
        //     }
        // }

        let mut tor: Vec<A> = Vec::new();

        if dist <= 0.0 {
            return tor;
        }

        let disc_dist = (dist/self.discretization).floor() as i64;
        let disc_pos = self.discretize(&pos);
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

        //println!("punti: \n-{}-{}-{}-{}\n", min_i, max_i, min_j, max_j);

        let loc = pos.clone();

        for i in min_i..max_i {
            for j in min_j..max_j {
                let bag_id = Int2D {
                    x: t_transform(i, max_x),
                    y: t_transform(j, max_y),
                };
                //println!("bag_id {} {}", bag_id.x, bag_id.y);
                if !self.fbag.contains_key(&bag_id) {
                    continue;
                }
                let check = check_circle(&bag_id, self.discretization, self.width, self.heigth, &pos, dist, self.toroidal);
                //TODO remove to vec
                let vector =  match self.fbag.get(&bag_id) {
                    Some(i) => i.to_vec(),
                    None => panic!("errore vettore fbag"),
                };
                if check == 1 {
                    for elem in vector {
                        //println!("conteggio -- i:{} j:{}", i, j);
                        //clone elem
                        tor.push(elem);
                    }
                } else if check == 0 {
                    for elem in vector {
                        //println!("conteggio 2-- i:{} j:{}", i, j);
                        //println!("dist {}  -- {}", distance(&loc, &(elem.clone().get_location()), self.width, self.heigth, self.toroidal), dist);
                        if distance(&loc, &(elem.get_location()), self.width, self.heigth, self.toroidal) <= dist {
                            //elem clone
                            tor.push(elem);
                        }
                    }

                }
            }
        }


        // //let x = (self.vec.len()/100)*10;
        // for y in 1..self.vec.len() {
        //     tor.push(self.vec[y].clone())
        // }
        // //let x = self.vec.clone();
        // //x
        // // for i in 0..3 {
        // //     vec2.push(x[i]);
        // // }
        tor
    }

    pub fn get_objects_at_location(&self, pos: Real2D) -> Vec<&A>{
        let bag = self.discretize(&pos);
        let mut result = Vec::new();
        for (key, val) in self.fbag.iter() {
            if *key == bag {
                for elem in val{
                    result.push(elem);
                }
            }
        }
        result

    }

    pub fn num_objects(&self) -> usize {
        self.findex.len()
    }

    pub fn num_objects_at_location(&self, pos: Real2D) -> usize {
        let bag = self.discretize(&pos);
        let mut result = Vec::new();
        for (key, val) in self.fbag.iter() {
            if *key == bag {
                for elem in val {
                    result.push(elem);
                }
            }
        }
        result.len()
    }

    pub fn get_object_location(&self, obj: A) -> Option<&Real2D> {
        self.fpos.get(&obj)
    }

    fn discretize(&self, pos: &Real2D) -> Int2D {
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
        n%size
    } else {
        (n%size) + size
    }
}

fn check_circle(bag: &Int2D, discretization: f64,width: f64, heigth: f64, pos: &Real2D, dis: f64, tor: bool) -> i8{
    let nw = Real2D {
        x: (bag.x as f64)*discretization,
        y: (bag.y as f64)*discretization,
    };
    let ne = Real2D {
        x: nw.x,
        y: (nw.y + discretization).min(heigth),
    };
    let sw = Real2D {
        x: (nw.x + discretization).min(width),
        y: nw.y,
    };
    let se = Real2D {
        x: sw.x,
        y: ne.y,
    };

    if distance(&nw, &pos, width, heigth, tor) <= dis &&
        distance(&ne, &pos, width, heigth, tor) <= dis &&
         distance(&sw, &pos, width, heigth, tor) <= dis &&
          distance(&se, &pos, width, heigth, tor) <= dis {
              1
    } else if distance(&nw, &pos, width, heigth, tor) > dis &&
               distance(&ne, &pos, width, heigth, tor) > dis &&
                distance(&sw, &pos, width, heigth, tor) > dis &&
                 distance(&se, &pos, width, heigth, tor) > dis {
                   -1
    } else {
        0
    }
}

fn distance(pos1: &Real2D, pos2: &Real2D, dim1: f64, dim2: f64, tor: bool) -> f64{

    let dx;
    let dy;

    if tor {
        dx = toroidal_distance(pos1.x, pos2.x, dim1);
        dy = toroidal_distance(pos1.y, pos2.y, dim2);
    } else {
        dx = pos1.x - pos2.x;
        dy = pos1.y - pos2.y;
    }
    (dx*dx + dy*dy).sqrt()
}

fn toroidal_distance(val1: f64, val2: f64, dim: f64) -> f64{

    if (val1 - val2).abs() <= dim/2.0 {
        return val1 - val2;
    }

    let d = toroidal_transform(val1, dim) - toroidal_transform(val2, dim);

    if d*2.0 > dim {
        d - dim
    } else if d*2.0 < dim {
        d + dim
    } else {
        d
    }
}

fn toroidal_transform(val: f64, dim: f64) -> f64 {

    if val >= 0.0 && val < dim {
        val
    } else {
        let val = val%dim;
        if val < 0.0 {
            let _val = val + dim;
        }
        val
    }
}
