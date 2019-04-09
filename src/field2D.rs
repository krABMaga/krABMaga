//use crate::location::Location;
use crate::location::Location2D;

pub struct Field2D<P: Location2D> {
    pub vec : Vec<P>,
}

impl <P: Location2D> Field2D<P> {
    pub fn new() -> Field2D<P> {
        Field2D {
            vec: Vec::new(),
        }
    }

    pub fn set_object_location(&mut self, object: P) {
        self.vec.push(object);
    }

    pub fn get_neighbors_within_distance(&self, object: P) -> Vec<&P>{
        let mut vec = Vec::new();
        let x = (self.vec.len()/100)*10;
        for i in 0..x {
            let elem = &self.vec[i];
            vec.push(elem);
        }
        vec
    }
    fn get_objects_at_location(){

    }
    fn num_objects_at_location() {

    }
    fn get_object_location() {

    }
}
