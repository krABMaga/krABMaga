use std::any::Any;

use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;

#[derive(Default)]
struct DummyState;

impl State for DummyState {
    fn init(&mut self, _schedule: &mut Schedule) {}

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn reset(&mut self) {}

    fn update(&mut self, _step: u64) {}
}

#[test]
fn agent_defaults_return_empty_hooks() {
    use crate::utils::mynode::MyNode;

    use krabmaga::engine::agent::Agent;

    let mut agent = MyNode { id: 1, flag: false };
    let mut state = DummyState;

    assert!(!agent.is_stopped(&mut state));
    assert!(agent.before_step(&mut state).is_none());
    assert!(agent.after_step(&mut state).is_none());
}
