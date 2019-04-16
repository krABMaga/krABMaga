use crate::field2D::Field2D;

pub struct MyData{
    pub field1: Field2D,
    pub field2: Field2D,
    pub field3: Field2D,
}

impl MyData{
    pub fn new() -> MyData {
        MyData {
            field1: Field2D::new(),
            field2: Field2D::new(),
            field3: Field2D::new(),
        }
    }
}

pub trait Agent {
    fn step(self, data: &MyData);
}
