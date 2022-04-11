// Must be implemented for each state for Model Exploration

use crate::engine::state::State;

pub trait ExploreState {
    fn new_with_parameters(parameters: &str) -> dyn State;

    /*Optional functions*/
    // #[allow(unused_variables)]
}
