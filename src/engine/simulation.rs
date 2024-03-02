use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::log::LogPlugin;
use bevy::prelude::*;

use crate::engine::fields::field_2d::{update_field, Field2D};
use crate::engine::resources::engine_configuration::EngineConfiguration;
use crate::engine::rng::RNG;
use crate::engine::systems::double_buffer_sync::double_buffer_sync;
use crate::engine::systems::engine_config_update::engine_config_update;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum SimulationSet {
    BeforeStep,
    Step,
    AfterStep,
}

pub struct Simulation {
    app: App,
    steps: Option<u32>,
}

impl Simulation {
    pub fn build() -> Self {
        let mut app = App::new();
        #[cfg(feature = "trace_tracy")]
        app.add_plugins(LogPlugin::default());
        app.configure_sets(
            Update,
            (
                SimulationSet::BeforeStep,
                SimulationSet::Step,
                SimulationSet::AfterStep,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (engine_config_update,).in_set(SimulationSet::BeforeStep),
        );

        Self { app, steps: None }
    }

    pub fn with_num_threads(mut self, num_threads: usize) -> Self {
        self.app.add_plugins(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                // Assign all threads to compute
                compute: TaskPoolThreadAssignmentPolicy {
                    // set the minimum # of compute threads
                    // to the total number of available threads
                    min_threads: num_threads,
                    max_threads: num_threads, // unlimited max threads
                    percent: 1.0,             // this value is irrelevant in this case
                },
                ..default()
            },
        });
        self
    }

    // TODO expose a macro to wrap a fn describing the step of one agent and transform it in a system that cycles all agents? This is probably the worst aspect of the refactor, the step signature can easily get too complex to read.
    pub fn register_step_handler<Params>(
        mut self,
        step_handler: impl IntoSystemConfigs<Params>,
    ) -> Self {
        self.app
            .add_systems(Update, (step_handler,).in_set(SimulationSet::Step));
        self
    }

    // TODO figure out a way to automatically register double buffers
    pub fn register_double_buffer<T: Component + Copy + Send>(mut self) -> Self {
        self.app.add_systems(
            Update,
            (double_buffer_sync::<T>,).in_set(SimulationSet::BeforeStep),
        );
        self
    }

    pub fn with_steps(mut self, steps: u32) -> Self {
        self.steps = Some(steps);
        self
    }

    // TODO specify this is required (SimulationBuilder with validation, which generates a Simulation on build()?)
    pub fn with_engine_configuration(mut self, config: EngineConfiguration) -> Self {
        self.app.insert_resource(config);
        self
    }

    pub fn with_rng(mut self, seed: u64) -> Self {
        let rng = RNG::new(seed, 0);
        self.app.insert_resource(rng);
        self
    }

    pub fn add_field(&mut self, field: Field2D<Entity>) -> &mut Simulation {
        self.app.world.spawn((field,));
        self.app
            .add_systems(Update, (update_field,).in_set(SimulationSet::BeforeStep));
        self
    }

    pub fn run(mut self) {
        match self.steps {
            Some(steps) => {
                for _ in 0..steps {
                    self.app.update(); // TODO better approach? This seems to work fine but the example suggests a dedicated scheduler
                }
            }
            None => {
                println!("Running");
                self.app.run();
            }
        }
    }

    pub(crate) fn spawn_agent(&mut self) -> EntityWorldMut {
        self.app.world.spawn(())
    }
}
