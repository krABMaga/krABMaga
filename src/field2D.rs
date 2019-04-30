//use crate::location::Location;
use crate::location::Location2D;
use std::fmt::Display;

pub struct Field2D {
    pub vec : Vec<Box<dyn Location2D>>,
}

impl Field2D {
    pub fn new() -> Field2D {
        Field2D {
            vec: Vec::new(),
        }
    }

    pub fn set_object_location<'a, P: 'static + Location2D>(&mut self, object: P) {
        self.vec.push(Box::new(object));
        //print!("lunghezza vett {}", self.vec.len())
    }

    pub fn get_neighbors_within_distance<'a, P: Location2D>(&self, object: &'a P) -> Vec<&Box<dyn Location2D>>{
        let mut vec2 = Vec::new();
        //let x = (self.vec.len()/100)*10;
        for i in 0..3 {
            let elem = &self.vec[i];
            vec2.push(elem);
        }
        vec2
    }

    fn get_objects_at_location(){

    }
    fn num_objects_at_location() {

    }
    fn get_object_location() {

    }
}
