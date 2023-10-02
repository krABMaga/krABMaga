use crate::engine::{schedule::ScheduleOptions, state::State};

use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;

/// Agent define the specific functionalities that an agent of a simulation should have e.g. the step function
pub trait Agent: Downcast + DynClone + Send + Sync {
    /// Define the core behaviour of the agent. Write here all the code that will be executed by the agent at each step.
    ///
    /// # Arguments
    /// * `state` - state of the simulation
    fn step(&mut self, state: &mut dyn State);

    /// Specifies whether this agent should be removed from the schedule after the current step.
    ///
    /// # Arguments
    /// * `state` - state of the simulation
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }

    /// Define the optional behaviour of the agent before computing the actual step
    ///
    /// # Arguments
    /// * `state` - state of the simulation
    fn before_step(&mut self, _state: &mut dyn State) {}

    /// Define the optional behaviour of the agent after computing the actual step
    /// # Arguments
    /// * `state` - state of the simulation
    fn after_step(&mut self, _state: &mut dyn State) {}
}

/// Trait use to compare agents.
///
/// Must be implemented to use `check_reproducibility!` macro.
/// There aren't constraints about what must be compared by an agent because it depends on your model.
pub trait ReproducibilityEq {
    /// Function used to compare two agents.
    /// Return true if the agent are the same, false otherwise.
    fn equals(&self, other: &Self) -> bool;
}

dyn_clone::clone_trait_object!(Agent);
impl_downcast!(Agent);
