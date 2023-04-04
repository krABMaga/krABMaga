use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "visualization", feature = "visualization_wasm"))] {
        use std::time::Duration;

        use bevy::ecs::schedule::ShouldRun;
        use bevy::prelude::ResMut;

        use crate::visualization::utils::updated_time::Time;

        /// This util allows specifying the number of simulation steps to execute every second.
        /// The flow starts from the UI, where the user specifies a f32 representing the steps to run per
        /// second (by default, 1 step per second). When the simulation system runs, we check if we've
        /// accumulated at least a step, if we did, we consume as many steps as possible.
        /// TODO swap to https://github.com/bevyengine/bevy/pull/3002 when Bevy 0.6 is released.
        pub struct FixedTimestep;

        impl FixedTimestep {
            // pub fn step(mut time: ResMut<Time>, mut accumulator: ResMut<FixedTimestepState>) -> ShouldRun {
            pub fn step(mut time: Time, mut accumulator: FixedTimestepState) -> ShouldRun {
                if accumulator.sub_step().is_some() {
                    time.advance_step();
                    ShouldRun::YesAndCheckAgain
                } else {
                    ShouldRun::No
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct FixedTimestepState {
            time: Duration,
            steps: u32,
        }

        impl Default for FixedTimestepState {
            fn default() -> Self {
                Self {
                    time: Duration::from_secs(0),
                    steps: 0,
                }
            }
        }

        impl FixedTimestepState {
            pub fn new(time: Duration, steps: u32) -> Self {
                Self { time, steps }
            }

            /// The number of accrued steps.
            #[inline]
            pub fn steps(&self) -> u32 {
                self.steps
            }

            /// The amount of time accrued toward new steps as [`Duration`].
            #[inline]
            pub fn time(&self) -> Duration {
                self.time
            }

            pub fn set_steps(&mut self, steps: u32) {
                self.steps = steps;
            }

            pub fn set_time(&mut self, time: Duration) {
                self.time = time;
            }

            /// The amount of time accrued toward the next step as [`f32`] % of timestep.
            pub fn overstep_percentage(&self, timestep: Duration) -> f32 {
                self.time.as_secs_f32() / timestep.as_secs_f32()
            }

            /// The amount of time accrued toward the next step as [`f64`] % of timestep.
            pub fn overstep_percentage_f64(&self, timestep: Duration) -> f64 {
                self.time.as_secs_f64() / timestep.as_secs_f64()
            }

            /// Add to the accrued time, then convert into as many steps as possible.
            pub fn add_time(&mut self, time: Duration, timestep: Duration) {
                self.time += time;
                while self.time >= timestep {
                    self.time -= timestep;
                    self.steps += 1;
                }
            }

            /// Consume a stored step (if any).
            pub fn sub_step(&mut self) -> Option<u32> {
                let remaining = self.steps.checked_sub(1);
                self.steps = self.steps.saturating_sub(1);
                remaining
            }

            pub fn reset(&mut self) {
                self.time = Duration::from_secs(0);
                self.steps = 0;
            }
        }

    }
}
