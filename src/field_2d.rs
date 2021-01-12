use std::fmt::Display;
use crate::location::Real2D;
use std::hash::Hash;
use crate::field::DoubleBufferedField;
use crate::location::Int2D;
use crate::location::Location2D;
use std::cmp;
use crate::utils::dbdashmap::DBDashMap;



pub struct Field2D<A: Location2D<Real2D> + Clone + Hash + Eq + Display + Copy> {
    pub findex: DBDashMap<A, Int2D>,
    pub fbag: DBDashMap<Int2D, Vec<A>>,
    pub fpos: DBDashMap<A, Real2D>,
    pub width: f64,
    pub heigth: f64,
    pub discretization: f64,
    pub toroidal: bool,
}

impl<A: Location2D<Real2D> + Clone + Hash + Eq + Display + Copy> Field2D<A>  {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> Field2D<A> {
        Field2D {
            findex: DBDashMap::new(),
            fbag: DBDashMap::new(),
            fpos: DBDashMap::new(),
            width: w,
            heigth: h,
            discretization: d,
            toroidal: t,
        }
    }

    pub fn set_object_location(&self, object: A, pos: Real2D) {
        let bag = self.discretize(&pos);
        self.fpos.insert(object,pos);
        self.findex.insert(object,bag);
        match self.fbag.get_mut(&bag){
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

    pub fn get_neighbors_within_distance(&self, pos: Real2D, dist: f64) -> Vec<&A> {
        
        let density = (self.width * self.heigth)/f64::from(self.findex.r_len() as i32);
        let sdist = (dist * dist) as usize;
        let mut tor: Vec<&A> = Vec::with_capacity(density as usize * sdist);
        // let mut tor: Vec<&A> = Vec::new();
        
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

        for i in min_i..max_i+1 {
            for j in min_j..max_j+1 {
                let bag_id = Int2D {
                    x: t_transform(i, max_x),
                    y: t_transform(j, max_y),
                };

            //    if !self.r_fbag.contains_key(&bag_id) {
            //        continue;
            //    }

                let check = check_circle(&bag_id, self.discretization, self.width, self.heigth, &pos, dist, self.toroidal);
                let vector =  match self.fbag.get(&bag_id) {
                    Some(i) => i,
                    None => continue,
                };

                if check == 1 {
                    for elem in vector {
                        tor.push(elem);
                    }
                } else if check == 0 {
                    for elem in vector {
                        if distance(&pos, &(elem.get_location()), self.width, self.heigth, self.toroidal) <= dist {
                            tor.push(elem);
                        }
                    }
                }
            }
        }
        
        tor

    }


    pub fn get_objects_at_location(&self, pos: Real2D) -> Vec<&A>{
        let bag = self.discretize(&pos);
        let mut result = Vec::new();
        
        match self.fbag.get(&bag){
            Some(v) => { 
                for el in v{
                    result.push(el);
                }
            }
            None => ()
        }

        result
    }

    pub fn num_objects(&self) -> usize {
        self.findex.r_len()
    }

    pub fn num_objects_at_location(&self, pos: Real2D) -> usize {
        let bag = self.discretize(&pos);
        match self.fbag.get(&bag){
            Some(v) => { 
                v.len()
            }
            None => 0
        }
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

pub fn toroidal_distance(val1: f64, val2: f64, dim: f64) -> f64{

    if (val1 - val2).abs() <= dim/2.0 {
        return val1 - val2;
    }

    let d = toroidal_transform(val1, dim) - toroidal_transform(val2, dim);

    if d*2.0 > dim {
        d - dim
    } else if d*2.0 < -dim {
        d + dim
    } else {
        d
    }
}

pub fn toroidal_transform(val: f64, dim: f64) -> f64 {

    if val >= 0.0 && val < dim {
        val
    } else {
        let mut val = val%dim;
        if val < 0.0 {
            val = val + dim;
        }
        val
    }
}


impl<A: Location2D<Real2D> + Clone + Hash + Eq + Display + Copy> DoubleBufferedField for Field2D<A>{
    fn update(&mut self){
        self.fpos.update();
        self.fbag.update();
        self.findex.update();
    }
}



