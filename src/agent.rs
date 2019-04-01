use crate::simstate::SimState;

pub trait Agent{
    fn step<A: Agent>(self, simstate: &SimState<A>);
}
