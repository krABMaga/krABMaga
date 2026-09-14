use std::any::Any;
use std::sync::atomic::{AtomicUsize, Ordering};

use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use rayon::prelude::*;

static SAMPLE_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
struct TestState {
    score: f32,
}

impl State for TestState {
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

fn gen_sample(_state: &TestState) -> TestState {
    let index = SAMPLE_INDEX.fetch_add(1, Ordering::SeqCst);
    let score = if index.is_multiple_of(2) { 5.0 } else { 3.0 };

    TestState { score }
}

fn cost_function(state: &TestState) -> f32 {
    state.score
}

#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn random_search_picks_lowest_cost_sample() {
    SAMPLE_INDEX.store(0, Ordering::SeqCst);

    let init_state = TestState { score: 10.0 };
    let (best_state, cost) =
        krabmaga::random_search!(init_state, 1, 0.0, cost_function, gen_sample, 2, 0, 1);

    assert_eq!(cost, 3.0);
    assert_eq!(best_state.score, 3.0);
}
