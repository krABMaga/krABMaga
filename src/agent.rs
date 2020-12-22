use crate::state::State;

pub trait Agent {
    type SimState: State + Sync + Send;

    fn step(&mut self,state: &Self::SimState);
}