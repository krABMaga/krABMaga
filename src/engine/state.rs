///Types which may be used to hold simulation state data.
pub trait State{
    ///Updates the simulation state double buffer.
    fn update(&self){}
}