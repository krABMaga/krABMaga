//use crate::location::Location;
use crate::location::Location2D;

pub struct Field2D {
    pub vec : Vec<Box<dyn Location2D>>,
}

impl Field2D {
    pub fn new() -> Field2D {
        Field2D {
            vec: Vec::new(),
        }
    }

    pub fn set_object_location<P: 'static +  Location2D>(&mut self, object: P) {
        self.vec.push(Box::new(object));
    }

    pub fn get_neighbors_within_distance<P: 'static +  Location2D>(&self, object: P) {
        let mut vec = Vec::new();
        let x = (self.vec.len()/100)*10;
        println!("{}", x);
        for i in 0..x {
            let elem = &self.vec[i];
            vec.push(elem);
        }
    }

    fn get_objects_at_location(){

    }
    fn num_objects_at_location() {

    }
    fn get_object_location() {

    }
}
