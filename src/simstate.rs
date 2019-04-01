use crate::schedule::Schedule;
use crate::agent::Agent;

pub struct SimState<A: Agent + Clone> {
    schedule: Schedule<A>,

}
