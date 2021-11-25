use crate::engine::schedule::Schedule;
use std::any::Any;
use cfg_if::cfg_if;

cfg_if!{
    if #[cfg(feature ="parallel")] {
        //we need a specific type for the state
        pub trait State: Sync + Send + 'static {
            fn reset(&mut self);
            fn init(&mut self, schedule: &mut Schedule);
            fn as_any(&self) -> &dyn Any;
            fn as_state_mut(&mut self) -> &mut dyn State;
            fn as_state(&self) -> &dyn State;

            /**Optional functions**/
            #[allow(unused_variables)]
            fn update(&mut self, step: u64) {}
            #[allow(unused_variables)]
            fn before_step(&mut self, schedule: &mut Schedule) {}
            #[allow(unused_variables)]
            fn after_step(&mut self, schedule: &mut Schedule) {}
            #[allow(unused_variables)]
            fn end_condition(&mut self, schedule: &mut Schedule) -> bool {
                false
            }
        }
    } else{
        pub trait State: Send + 'static {
            fn reset(&mut self);
            fn init(&mut self, schedule: &mut Schedule);
            fn as_any(&self) -> &dyn Any;
            fn as_state_mut(&mut self) -> &mut dyn State;
            fn as_state(&self) -> &dyn State;

            /**Optional functions**/
            #[allow(unused_variables)]
            fn update(&mut self, step: u64) {}
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
