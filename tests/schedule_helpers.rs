use rust_ab::engine::agent::Agent;
use rust_ab::engine::schedule::{Schedule, ScheduleOptions};
use rust_ab::engine::state::State;
use std::collections::HashMap;
use std::hash::Hash;

#[test]
fn schedule_helpers() {
    let mut schedule = Schedule::new();
    let mut state = MyState {
        global_should_die: false,
    };

    schedule.schedule_repeating(
        MyAgent {
            should_die: false,
            should_reproduce: false,
            am_i_a_clone: false,
        },
        0.,
        0,
    );

    assert_eq!(schedule.events.lock().unwrap().len(), 1);
    // First step is taken. The first agent sets itself in reproduction mode and stands by.
    schedule.step(&mut state);
    println!("Step 1 finished!");
    // The first step has been taken and the agent reproduces. There are two agents now.
    assert_eq!(schedule.events.lock().unwrap().len(), 2);
    // Second step is taken. The first agent schedules its own death at the end of this step.
    // The second agent happily prints a message and stands by.
    schedule.step(&mut state);
    println!("Step 2 finished!");
    // The first agent has died, the clone remains.
    assert_eq!(schedule.events.lock().unwrap().len(), 1);
    // Kill the remaining agent through a flag placed on the state.
    state.global_should_die = true;
    schedule.step(&mut state);
    println!("Step 3 finished!");
    // The clone was killed by the flag on the state.
    assert_eq!(schedule.events.lock().unwrap().len(), 0);
}

pub struct MyState {
    pub global_should_die: bool,
}

impl State for MyState {}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MyAgent {
    pub should_die: bool,
    pub should_reproduce: bool,
    pub am_i_a_clone: bool,
}

impl Agent for MyAgent {
    type SimState = MyState;

    // Clones will simply print a message. The initial agent will clone itself after the first step
    // and die after the second.
    fn step(&mut self, _state: &Self::SimState) {
        if self.am_i_a_clone {
            println!("I'm a living clone!");
        } else {
            // Don't reproduce if we're going to die in this step
            if !self.should_die {
                println!("Scheduling reproduction!");
                self.should_reproduce = true;
            }
        }
    }

    fn should_remove(&mut self, state: &Self::SimState) -> bool {
        self.should_die || state.global_should_die
    }

    fn should_reproduce(
        &mut self,
        _state: &Self::SimState,
    ) -> Option<HashMap<Box<Self>, ScheduleOptions>> {
        if self.should_reproduce {
            self.should_reproduce = false;
            self.should_die = true; // Die after the next step

            let new_agent = MyAgent {
                should_die: false,
                should_reproduce: false,
                am_i_a_clone: true,
            };

            let schedule_options = ScheduleOptions {
                ordering: 0,
                repeating: true,
            };

            let mut hash_map = HashMap::new();
            hash_map.insert(Box::new(new_agent), schedule_options);
            Some(hash_map)
        } else {
            None
        }
    }
}
