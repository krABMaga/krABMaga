use std::collections::HashMap;

use crate::engine::schedule::ScheduleOptions;
use crate::engine::state::State;

pub trait Agent {
    type SimState: State + Sync + Send;

    fn step(&mut self, state: &Self::SimState);

    /// Specifies whether this agent should be removed from the schedule after the current step.
    fn should_remove(&mut self, _state: &Self::SimState) -> bool {
        false
    }

    /// Allows the agent to schedule new agents without having direct access to the Schedule.
    /// This should NOT return an agent that is already scheduled.
    fn should_reproduce(
        &mut self,
        _state: &Self::SimState,
    ) -> Option<HashMap<Box<Self>, ScheduleOptions>> {
        None
    }
}
