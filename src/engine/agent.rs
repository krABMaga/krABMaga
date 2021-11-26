use crate::engine::{schedule::ScheduleOptions, state::State};

use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;

pub trait Agent: Downcast + DynClone + Send + Sync {
    fn step(&mut self, state: &mut dyn State);

    fn get_id(&self) -> u32;

    // Specifies whether this agent should be removed from the schedule after the current step.
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }

    fn before_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }

    fn after_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }
}

dyn_clone::clone_trait_object!(Agent);
impl_downcast!(Agent);
