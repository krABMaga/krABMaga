use crate::engine::schedule::Schedule;
use cfg_if::cfg_if;
use std::any::Any;

cfg_if! {
    if #[cfg(feature ="parallel")] {
        //we need a specific type for the state
        pub trait State: Sync + Send + 'static {

            fn init(&mut self, schedule: &mut Schedule);
            fn as_any(&self) -> &dyn Any;
            fn as_any_mut(&mut self) -> &mut dyn Any;
            fn as_state_mut(&mut self) -> &mut dyn State;
            fn as_state(&self) -> &dyn State;

            /**Optional functions**/
            fn reset(&mut self) {}
            fn update(&mut self, _step: u64) {}
            fn before_step(&mut self, _schedule: &mut Schedule) {}
            fn after_step(&mut self, _schedule: &mut Schedule) {}
            fn end_condition(&mut self) -> bool {
                false
            }
        }
    } else{
        /// Trait to define basic function for a simulation state.
        ///
        /// * `init` - should initialize all the starting values of a simulation
        ///
        /// * `as_any`/`as_any_mut`/`as_state`/`as_state_mut` - support functions to return a `dyn` values
        ///
        /// * `reset` - should reset all the values of the simulation
        ///
        /// * `update` -  function to wrap up the calls on fields update
        ///
        /// * `before_step` - define the optional behaviour of the state before computing the actual step
        ///
        /// * `after_step` - define the optional behaviour of the state after computing the actual step
        ///
        /// * `end_condition` - define a condition where the simulation should end
        pub trait State: Send + 'static {

            fn init(&mut self, schedule: &mut Schedule);
            fn as_any_mut(&mut self) -> &mut dyn Any;
            fn as_any(&self) -> &dyn Any;
            fn as_state_mut(&mut self) -> &mut dyn State;
            fn as_state(&self) -> &dyn State;
            fn reset(&mut self);
            fn update(&mut self, step: u64);
            /**Optional functions**/
            #[allow(unused_variables)]
            fn before_step(&mut self, schedule: &mut Schedule) {}
            #[allow(unused_variables)]
            fn after_step(&mut self, schedule: &mut Schedule) {}
            #[allow(unused_variables)]
            fn end_condition(&mut self, schedule: &mut Schedule) -> bool {
                false
            }
        }
    }
}
