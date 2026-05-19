#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn schedule_operations() {
    use krabmaga::engine::schedule::Schedule;

    use crate::utils::mynode::MyNode;

    let mut schedule = Schedule::new();
    let node1 = MyNode { id: 0, flag: false };
    let node2 = MyNode { id: 1, flag: false };

    schedule.schedule_repeating(Box::new(node1), 0., 0);
    schedule.schedule_repeating(Box::new(node2), 0., 0);

    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 2);

    for (i, a) in agents.iter().enumerate() {
        assert_eq!(
            *a.downcast_ref::<MyNode>().unwrap(),
            if i == 0 { node1 } else { node2 }
        );
    }

    assert!(schedule.dequeue(Box::new(node1), node1.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 1);
    let a = agents[0].downcast_ref::<MyNode>().unwrap();
    assert_eq!(*a, node2);

    assert!(schedule.dequeue(Box::new(node2), node2.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 0);
}

#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn distributed_schedule_operations() {
    use krabmaga::engine::schedule::Schedule;

    use crate::utils::mynode::MyNode;

    let mut schedule = Schedule::new();
    let node1 = MyNode { id: 0, flag: false };
    let node2 = MyNode { id: 1, flag: false };

    let (id1, opt1) = schedule.distributed_schedule_repeating(Box::new(node1), 0., 0);
    let (id2, opt2) = schedule.distributed_schedule_repeating(Box::new(node2), 0., 0);

    assert_eq!(id1, 0);
    assert!(opt1);

    assert_eq!(id2, 1);
    assert!(opt2);

    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 2);

    for (i, a) in agents.iter().enumerate() {
        assert_eq!(
            *a.downcast_ref::<MyNode>().unwrap(),
            if i == 0 { node1 } else { node2 }
        );
    }

    assert!(schedule.dequeue(Box::new(node1), node1.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 1);
    let a = agents[0].downcast_ref::<MyNode>().unwrap();
    assert_eq!(*a, node2);

    assert!(schedule.dequeue(Box::new(node2), node2.id));
    let agents = schedule.get_all_events();
    assert_eq!(agents.len(), 0);
}

#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn schedule_default_matches_new() {
    use krabmaga::engine::schedule::Schedule;

    let schedule: Schedule = Default::default();
    assert_eq!(schedule.step, 0);
    assert_eq!(schedule.time, 0.0);
    assert!(schedule.get_all_events().is_empty());
}

#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn dequeue_returns_false_for_missing_agent() {
    use krabmaga::engine::agent::Agent;
    use krabmaga::engine::schedule::Schedule;
    use krabmaga::engine::schedule::ScheduleOptions;
    use krabmaga::engine::state::State;

    #[derive(Clone)]
    struct TestAgent;

    impl Agent for TestAgent {
        fn step(&mut self, _state: &mut dyn State) {}

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

    let mut schedule = Schedule::new();
    let removed = schedule.dequeue(Box::new(TestAgent), 42);
    assert!(!removed);
}

#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn step_with_empty_queue_updates_state() {
    use krabmaga::engine::schedule::Schedule;
    use krabmaga::engine::state::State;
    use std::any::Any;

    #[derive(Default)]
    struct DummyState {
        updates: usize,
        befores: usize,
        afters: usize,
    }

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

        fn update(&mut self, _step: u64) {
            self.updates += 1;
        }

        fn before_step(&mut self, _schedule: &mut Schedule) {
            self.befores += 1;
        }

        fn after_step(&mut self, _schedule: &mut Schedule) {
            self.afters += 1;
        }
    }

    let mut schedule = Schedule::new();
    let mut state = DummyState::default();

    schedule.step(&mut state);

    assert_eq!(state.befores, 1);
    assert_eq!(state.afters, 1);
    assert_eq!(state.updates, 2);
    assert_eq!(schedule.step, 1);
}

#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn step_breaks_on_future_time_events() {
    use krabmaga::engine::agent::Agent;
    use krabmaga::engine::agentimpl::AgentImpl;
    use krabmaga::engine::schedule::Schedule;
    use krabmaga::engine::schedule::ScheduleOptions;
    use krabmaga::engine::state::State;
    use std::any::Any;

    #[derive(Clone)]
    struct TestAgent;

    impl Agent for TestAgent {
        fn step(&mut self, _state: &mut dyn State) {}

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

    let mut schedule = Schedule::new();
    let mut state = DummyState::default();

    schedule.schedule_once(AgentImpl::new(Box::new(TestAgent), 1), 0.0, 0);
    schedule.schedule_once(AgentImpl::new(Box::new(TestAgent), 2), 1.0, 0);

    schedule.step(&mut state);

    assert_eq!(schedule.get_all_events().len(), 1);
    assert_eq!(schedule.step, 1);
}
