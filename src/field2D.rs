use crate::location::Location2D;

#[derive(Clone, Debug)]
pub struct Field2D<A: Location2D + Clone> {
    pub vec : Vec<A>,
}

impl<A: Location2D + Clone> Field2D<A> {
    pub fn new() -> Field2D<A> {
        Field2D {
            vec: Vec::new(),
        }
    }

    pub fn set_object_location(&mut self, object: A) {

        self.vec.push(object);
        //print!("lunghezza vett {}", self.vec.len())
    }

    pub fn get_neighbors_within_distance(&self, _object: &A) -> Vec<A> {
        let mut vec2: Vec<A> = Vec::new();
        //let x = (self.vec.len()/100)*10;
        for y in 1..self.vec.len() {
            vec2.push(self.vec[y].clone())
        }
        //let x = self.vec.clone();
        //x
        // for i in 0..3 {
        //     vec2.push(x[i]);
        // }
        vec2
    }

    fn get_objects_at_location(){

    }
    fn num_objects_at_location() {

    }
    fn get_object_location() {

    }
}
