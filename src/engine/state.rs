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
    } else if #[cfg(any(feature = "visualization", feature = "visualization_wasm"))] {
        pub trait State: Sync + Send + 'static {

            fn init(&mut self, schedule: &mut Schedule);
            fn as_any(&self) -> &dyn Any;
            fn as_state_mut(&mut self) -> &mut dyn State;
            fn as_state(&self) -> &dyn State;
            fn reset(&mut self);

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

            fn init(&mut self, schedule: &mut Schedule);
            fn as_any_mut(&mut self) -> &mut dyn Any;
            fn as_any(&self) -> &dyn Any;
            fn as_state_mut(&mut self) -> &mut dyn State;
            fn as_state(&self) -> &dyn State;

            /**Optional functions**/
            fn reset(&mut self) {}
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
