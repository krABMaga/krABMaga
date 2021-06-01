use bevy::prelude::ResMut;

use crate::engine::schedule::Schedule;
use crate::visualization::renderable::Render;

/// The simulation system steps the schedule once per frame, effectively synchronizing frames and schedule steps.
pub fn simulation_system<A: Render + Clone>(
    mut schedule: ResMut<Schedule<A>>,
    mut state: ResMut<A::SimState>,
) {
    schedule.step(&mut *state);
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use bevy::ecs::prelude::IntoSystem;
    use bevy::prelude::{Stage, Transform};
    use bevy::prelude::{SystemStage, World};

    use crate::bevy::prelude::Visible;
    use crate::engine::agent::Agent;
    use crate::engine::schedule::Schedule;
    use crate::engine::state::State;
    use crate::visualization::renderable::{Render, SpriteType};
    use crate::visualization::systems::simulation_system::simulation_system;

    struct BasicState {
        pub stepped: RwLock<bool>,
    }
    impl State for BasicState {}

    #[derive(Copy, Clone)]
    struct BasicAgent;
    impl Agent for BasicAgent {
        type SimState = BasicState;

        fn step(&mut self, state: &BasicState) {
            let mut state_stepped = state.stepped.write().unwrap();
            *state_stepped = true;
        }
    }

    impl Render for BasicAgent {
        fn sprite(&self) -> SpriteType {
            SpriteType::Emoji(String::from("bird"))
        }

        fn position(&self, _state: &BasicState) -> (f32, f32, f32) {
            (0., 0., 0.)
        }

        fn scale(&self) -> (f32, f32) {
            (1., 1.)
        }

        fn rotation(&self) -> f32 {
            0.
        }

        fn update(
            &mut self,
            _transform: &mut Transform,
            _state: &BasicState,
            _visible: &mut Visible,
        ) {
        }
    }

    /// A simple test that sets up a basic state, agent and schedule, then schedules the single agent.
    /// The simulation_system is executed once. We check if the simulation_system correctly made the
    /// RustAB schedule step once. We do so by checking a boolean var on the state, which should be set
    /// to true if the agent has stepped.
    #[test]
    fn agent_setup() {
        // Setup resources
        let state = BasicState {
            stepped: RwLock::new(false),
        };
        let mut schedule = Schedule::<BasicAgent>::new();
        let agent = BasicAgent;
        schedule.schedule_repeating(agent, 0., 0);

        // Setup world
        let mut world = World::default();

        // Setup stage with a system
        let mut stage = SystemStage::parallel();
        stage.add_system(simulation_system::<BasicAgent>.system());

        // Insert resources
        world.insert_resource(state);
        world.insert_resource(schedule);

        // Run systems
        stage.run(&mut world);

        // The state 'stepped' bool should have been set to true by the agent stepping
        let state = world.get_resource::<BasicState>().unwrap();
        let stepped = state.stepped.read().unwrap();
        assert_eq!(*stepped, true);
    }
}
