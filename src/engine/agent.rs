use crate::engine::{schedule::ScheduleOptions, state::State};

use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;

/// Agent define the specific functionalities that an agent of a simulation should have e.g. the step function
pub trait Agent: Downcast + DynClone + Send + Sync {
    ///define the core behaviour of the agent
    fn step(&mut self, state: &mut dyn State);

    ///return the id of the agent
    fn get_id(&self) -> u32;

    /// Specifies whether this agent should be removed from the schedule after the current step.
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }

    ///define the optional behaviour of the agent before computing the actual step
    fn before_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }

    ///define the optional behaviour of the agent after computing the actual step
    fn after_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }
}

dyn_clone::clone_trait_object!(Agent);
impl_downcast!(Agent);
