use crate::simstate::SimState;

pub trait Agent{
    fn step<A: Agent + Clone>(self, simstate: &SimState<A>);
}
