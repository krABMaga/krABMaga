use crate::schedule::Schedule;
use crate::agent::Agent;

pub struct SimState<A: Agent> {
    schedule: Schedule<A>,

}
