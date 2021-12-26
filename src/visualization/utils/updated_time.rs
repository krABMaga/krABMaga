/// This module is mostly taken from this PR: https://github.com/bevyengine/bevy/pull/3002
/// with slight fixes to add compatibility with Bevy 0.5, as well as a safety check in the time system
/// to prevent the app from freezing when the steps per second value is too high.

use std::time::{Duration, Instant};
use bevy::prelude::ResMut;
use crate::visualization::utils::fixed_timestep::FixedTimestepState;


/// Tracks time elapsed since the previous update and since the app was started.
#[derive(Debug, Clone)]
pub struct Time {
    delta: Duration,
    delta_seconds: f32,
    delta_seconds_f64: f64,
    raw_delta: Duration,
    raw_delta_seconds: f32,
    raw_delta_seconds_f64: f64,
    relative_speed: f32,
    elapsed_since_startup: Duration,
    first_update: Option<Instant>,
    last_update: Option<Instant>,
    startup: Instant,
    fixed_delta: Duration,
    fixed_delta_seconds: f32,
    fixed_delta_seconds_f64: f64,
    fixed_elapsed_since_startup: Duration,
}

impl Default for Time {
    fn default() -> Self {
        // 60Hz is a popular number, but it can't be expressed as an exact float.
        // Powers of two (i.e. 32Hz, 64Hz, 128Hz) are better for numerical integration.
        let fixed_delta = Duration::from_secs_f32(1.0 / 64.0);
        Self {
            delta: Duration::from_secs(0),
            delta_seconds: 0.0,
            delta_seconds_f64: 0.0,
            raw_delta: Duration::from_secs(0),
            raw_delta_seconds: 0.0,
            raw_delta_seconds_f64: 0.0,
            relative_speed: 1.0,
            elapsed_since_startup: Duration::from_secs(0),
            first_update: None,
            last_update: None,
            startup: Instant::now(),
            fixed_delta,
            fixed_delta_seconds: fixed_delta.as_secs_f32(),
            fixed_delta_seconds_f64: fixed_delta.as_secs_f64(),
            fixed_elapsed_since_startup: Duration::from_secs(0),
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        let now = Instant::now();
        self.update_with_instant(now);
    }

    pub(crate) fn update_with_instant(&mut self, instant: Instant) {
        if let Some(last_update) = self.last_update {
            self.raw_delta = instant - last_update;
            self.raw_delta_seconds = self.raw_delta.as_secs_f32();
            self.raw_delta_seconds_f64 = self.raw_delta.as_secs_f64();

            if self.relative_speed != 1.0 {
                // just to be safe
                self.delta = self.raw_delta.mul_f64(self.relative_speed_f64());
            } else {
                self.delta = self.raw_delta;
            }

            self.delta_seconds = self.delta.as_secs_f32();
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.elapsed_since_startup += self.delta;
        } else {
            self.first_update = Some(instant);

            if self.relative_speed != 1.0 {
                // just to be safe
                self.elapsed_since_startup =
                    (instant - self.startup).mul_f64(self.relative_speed as f64);
            } else {
                self.elapsed_since_startup = instant - self.startup;
            }
        }

        self.last_update = Some(instant);
    }

    pub(crate) fn advance_step(&mut self) {
        self.fixed_elapsed_since_startup += self.fixed_delta;
    }

    /// The time since the last update, as a [`Duration`].
    /// Affected by [`Time::relative_speed`].
    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// The time since the last update, as [`f32`] seconds.
    /// Affected by [`Time::relative_speed`].
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The time since the last update, as [`f64`] seconds.
    /// Affected by [`Time::relative_speed`].
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// The exact CPU time since the last update, as a [`Duration`].
    /// **Not** affected by [`Time::relative_speed`].
    #[inline]
    pub fn raw_delta(&self) -> Duration {
        self.raw_delta
    }

    /// The exact CPU time since the last update, as [`f32`] seconds.
    /// **Not** affected by [`Time::relative_speed`].
    #[inline]
    pub fn raw_delta_seconds(&self) -> f32 {
        self.raw_delta_seconds
    }

    /// The exact CPU time since the last update, as [`f64`] seconds.
    /// **Not** affected by [`Time::relative_speed`].
    #[inline]
    pub fn raw_delta_seconds_f64(&self) -> f64 {
        self.raw_delta_seconds_f64
    }

    /// The reciprocal of the rate set by [`Time::set_steps_per_second`], as a [`Duration`].
    ///
    /// A fixed timestep only guarantees that N steps run for each second of elapsed time, not that
    /// one step runs every [`Time::fixed_delta`] seconds. The exact CPU time between steps depends
    /// on the frame rate, so use [`Time::fixed_delta`] in systems for consistent behavior.
    #[inline]
    pub fn fixed_delta(&self) -> Duration {
        self.fixed_delta
    }

    /// The reciprocal of the rate set by [`Time::set_steps_per_second`], as [`f32`] seconds.
    ///
    /// A fixed timestep only guarantees that N steps run for each second of elapsed time, not that
    /// one step runs every [`Time::fixed_delta`] seconds. The exact CPU time between steps depends
    /// on the frame rate, so use [`Time::fixed_delta`] in systems for consistent behavior.
    #[inline]
    pub fn fixed_delta_seconds(&self) -> f32 {
        self.fixed_delta_seconds
    }

    /// The reciprocal of the rate set by [`Time::set_steps_per_second`], as [`f64`] seconds.
    ///
    /// A fixed timestep only guarantees that N steps run for each second of elapsed time, not that
    /// one step runs every [`Time::fixed_delta`] seconds. The exact CPU time between steps depends
    /// on the frame rate, so use [`Time::fixed_delta`] in systems for consistent behavior.
    #[inline]
    pub fn fixed_delta_seconds_f64(&self) -> f64 {
        self.fixed_delta_seconds_f64
    }

    /// Change the step Hz (changes [`Time::fixed_delta`]).
    /// Use [`Time::set_relative_speed`] to ensure that systems using [`fixed_delta`] behave consistently.
    pub fn set_steps_per_second(&mut self, rate: f32) {
        self.fixed_delta = Duration::from_secs_f32(1.0 / rate);
        self.fixed_delta_seconds = self.fixed_delta.as_secs_f32();
        self.fixed_delta_seconds_f64 = self.fixed_delta.as_secs_f64();
    }

    /// The rate that time advances relative to CPU time, as [`f32`]. 1.0 by default.
    #[inline]
    pub fn relative_speed(&self) -> f32 {
        self.relative_speed
    }

    /// The rate that time advances relative to CPU time, as [`f64`]. 1.0 by default.
    #[inline]
    pub fn relative_speed_f64(&self) -> f64 {
        self.relative_speed as f64
    }

    /// Set the rate that time advances relative to CPU time. Must be >= 0.0.
    #[inline]
    pub fn set_relative_speed(&mut self, relative_speed: f32) {
        assert!((relative_speed >= 0.0) && relative_speed.is_finite());
        self.relative_speed = relative_speed;
    }

    /// The [`Instant`] when [`Time::update`] was first called, if it exists.
    #[inline]
    pub fn first_update(&self) -> Option<Instant> {
        self.first_update
    }

    /// The [`Instant`] when [`Time::update`] was last called, if it exists.
    #[inline]
    pub fn last_update(&self) -> Option<Instant> {
        self.last_update
    }

    /// The [`Instant`] the app was started.
    #[inline]
    pub fn startup(&self) -> Instant {
        self.startup
    }

    /// The time since startup, as [`Duration`].
    /// Advances by [`Time::delta`] each update.
    #[inline]
    pub fn elapsed_since_startup(&self) -> Duration {
        self.elapsed_since_startup
    }

    /// The time since startup, as [`f32`] seconds.
    /// Advances by [`Time::delta`] each update.
    #[inline]
    pub fn seconds_since_startup(&self) -> f32 {
        self.elapsed_since_startup().as_secs_f32()
    }

    /// The time since startup, as [`f64`] seconds.
    /// Advances by [`Time::delta`] each update.
    #[inline]
    pub fn seconds_since_startup_f64(&self) -> f64 {
        self.elapsed_since_startup().as_secs_f64()
    }

    /// The exact CPU time since startup, as [`Duration`].
    /// Advances by [`Time::raw_delta`] each update.
    pub fn raw_elapsed_since_startup(&self) -> Duration {
        self.last_update.unwrap_or(self.startup) - self.startup
    }

    /// The exact CPU time since startup, as [`f32`] seconds.
    /// Advances by [`Time::raw_delta`] each update.
    #[inline]
    pub fn raw_seconds_since_startup(&self) -> f32 {
        self.raw_elapsed_since_startup().as_secs_f32()
    }

    /// The exact CPU time since startup, as [`f64`] seconds.
    /// Advances by [`Time::raw_delta`] each update.
    #[inline]
    pub fn raw_seconds_since_startup_f64(&self) -> f64 {
        self.raw_elapsed_since_startup().as_secs_f64()
    }

    /// The time since startup, as [`Duration`].
    /// Advances by [`Time::fixed_delta`] each step.
    #[inline]
    pub fn fixed_elapsed_since_startup(&self) -> Duration {
        // NOTE: actually lags behind time.elapsed_since_startup()
        // because time.first_update() - time.startup() isn't counted
        self.fixed_elapsed_since_startup
    }

    /// The time since startup, as [`f32`] seconds.
    /// Advances by [`Time::fixed_delta`] each step.
    #[inline]
    pub fn fixed_seconds_since_startup(&self) -> f32 {
        self.fixed_elapsed_since_startup().as_secs_f32()
    }

    /// The time since startup, as [`f64`] seconds.
    /// Advances by [`Time::fixed_delta`] each step.
    #[inline]
    pub fn fixed_seconds_since_startup_f64(&self) -> f64 {
        self.fixed_elapsed_since_startup().as_secs_f64()
    }
}

pub(crate) fn time_system(mut time: ResMut<Time>, mut accumulator: ResMut<FixedTimestepState>) {
    time.update();
    accumulator.add_time(
        time.delta().mul_f64(time.relative_speed_f64()),
        time.fixed_delta(),
    );
    // Safety check to avoid crashing the application if the user sets an extremely high
    // steps_per_second value and too many steps start to accumulate. In this case, we cut the
    // steps per seconds by half.
    if accumulator.steps() > 200 {
        let halved_fixed_delta_seconds = (1.0 / time.fixed_delta().as_secs_f32()) / 2.;
        time.set_steps_per_second(halved_fixed_delta_seconds);
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use std::time::{Duration, Instant};
    use super::Time;
    use crate::FixedTimestepState;
    use bevy_utils::{Duration, Instant};
    use crate::visualization::utils::fixed_timestep::FixedTimestepState;

    #[test]
    fn update_test() {
        let start_instant = Instant::now();

        // Create a `Time` for testing.
        let mut time = Time {
            startup: start_instant,
            ..Default::default()
        };

        // Ensure `time` was constructed correctly.
        assert_eq!(time.raw_delta(), Duration::from_secs(0));
        assert_eq!(time.raw_delta_seconds(), 0.0);
        assert_eq!(time.raw_delta_seconds_f64(), 0.0);
        assert_eq!(time.delta(), Duration::from_secs(0));
        assert_eq!(time.delta_seconds(), 0.0);
        assert_eq!(time.delta_seconds_f64(), 0.0);
        assert_eq!(time.first_update(), None);
        assert_eq!(time.last_update(), None);
        assert_eq!(time.startup(), start_instant);
        assert_eq!(time.relative_speed(), 1.0);
        assert_eq!(time.elapsed_since_startup(), Duration::from_secs(0));
        assert_eq!(time.raw_elapsed_since_startup(), Duration::from_secs(0));

        // Update `time` and check results.
        // The first update to `time` normally happens before other systems have run,
        // so the first delta doesn't appear until the second update.
        let first_update_instant = Instant::now();
        time.update_with_instant(first_update_instant);

        assert_eq!(time.raw_delta(), Duration::from_secs(0));
        assert_eq!(time.raw_delta_seconds(), 0.0);
        assert_eq!(time.raw_delta_seconds_f64(), 0.0);
        assert_eq!(time.delta(), Duration::from_secs(0));
        assert_eq!(time.delta_seconds(), 0.0);
        assert_eq!(time.delta_seconds_f64(), 0.0);
        assert_eq!(time.first_update(), Some(first_update_instant));
        assert_eq!(time.last_update(), Some(first_update_instant));
        assert_eq!(time.startup(), start_instant);
        assert_eq!(time.relative_speed(), 1.0);
        assert_eq!(
            time.elapsed_since_startup(),
            first_update_instant - start_instant
        );
        assert_eq!(
            time.raw_elapsed_since_startup(),
            first_update_instant - start_instant
        );

        // Update `time` again and check results.
        // At this point its safe to use time.delta().
        let second_update_instant = Instant::now();
        time.update_with_instant(second_update_instant);

        assert_eq!(
            time.raw_delta(),
            second_update_instant - first_update_instant
        );
        assert_eq!(time.raw_delta_seconds(), time.raw_delta().as_secs_f32());
        assert_eq!(time.raw_delta_seconds_f64(), time.raw_delta().as_secs_f64());
        assert_eq!(time.delta(), second_update_instant - first_update_instant);
        assert_eq!(time.delta_seconds(), time.delta().as_secs_f32());
        assert_eq!(time.delta_seconds_f64(), time.delta().as_secs_f64());
        assert_eq!(time.first_update(), Some(first_update_instant));
        assert_eq!(time.last_update(), Some(second_update_instant));
        assert_eq!(time.startup(), start_instant);
        assert_eq!(time.relative_speed(), 1.0);
        assert_eq!(
            time.elapsed_since_startup(),
            second_update_instant - start_instant
        );
        assert_eq!(
            time.raw_elapsed_since_startup(),
            second_update_instant - start_instant
        );

        // Make app time advance at 2x the rate of the system clock.
        time.set_relative_speed(2.0);
        // Update `time` again 1 second later.
        let elapsed = Duration::from_secs(1);
        let third_update_instant = second_update_instant + elapsed;
        time.update_with_instant(third_update_instant);

        // Since app is advancing 2x the system clock, expect elapsed time
        // to have advanced by twice the amount of raw CPU time.
        assert_eq!(time.relative_speed(), 2.0);
        assert_eq!(time.raw_delta(), elapsed);
        assert_eq!(time.raw_delta_seconds(), time.raw_delta().as_secs_f32());
        assert_eq!(time.raw_delta_seconds_f64(), time.raw_delta().as_secs_f64());
        assert_eq!(time.delta(), elapsed.mul_f32(2.0));
        assert_eq!(time.delta_seconds(), time.delta().as_secs_f32());
        assert_eq!(time.delta_seconds_f64(), time.delta().as_secs_f64());
        assert_eq!(time.first_update(), Some(first_update_instant));
        assert_eq!(time.last_update(), Some(third_update_instant));
        assert_eq!(time.startup(), start_instant);
        assert_eq!(
            time.elapsed_since_startup(),
            second_update_instant - start_instant + elapsed.mul_f32(2.0)
        );
        assert_eq!(
            time.raw_elapsed_since_startup(),
            second_update_instant - start_instant + elapsed
        );
    }

    #[test]
    fn fixed_timestep_test() {
        let start_instant = Instant::now();

        // Create a `Time` for testing.
        let mut time = Time {
            startup: start_instant,
            ..Default::default()
        };

        let fixed_delta = time.fixed_delta();

        // Create a `FixedTimestep for testing.
        let mut accumulator = FixedTimestepState::new(Duration::from_secs(0), 0);

        // Get the first update out of the way, so that
        // time.delta() will get a nonzero value next time.
        let first_update_instant = Instant::now();
        time.update_with_instant(first_update_instant);

        // Confirm that fixed delta didn't change on its own.
        assert_eq!(time.fixed_delta(), fixed_delta);
        assert_eq!(time.fixed_delta_seconds(), fixed_delta.as_secs_f32());
        assert_eq!(time.fixed_delta_seconds_f64(), fixed_delta.as_secs_f64());

        // Run for 10.5x the timestep.
        let ten = time.fixed_delta() * 10;
        let half = time.fixed_delta() / 2;
        let second_update_instant = first_update_instant + ten + half;
        time.update_with_instant(second_update_instant);
        accumulator.add_time(
            time.delta().mul_f64(time.relative_speed_f64()),
            time.fixed_delta(),
        );

        // Confirm that 10.5 steps have accumulated.
        assert_eq!(time.raw_delta(), ten + half);
        assert_eq!(time.delta(), time.raw_delta());
        assert_eq!(accumulator.steps(), 10);
        assert_eq!(accumulator.time(), half);

        assert_eq!(time.fixed_elapsed_since_startup(), Duration::from_secs(0));
        assert_eq!(
            time.fixed_seconds_since_startup(),
            Duration::from_secs(0).as_secs_f32()
        );
        assert_eq!(
            time.fixed_seconds_since_startup_f64(),
            Duration::from_secs(0).as_secs_f64()
        );

        // Consume accumulated steps and advanced the fixed time clock.
        while accumulator.sub_step().is_some() {
            time.advance_step();
        }

        // Confirm that fixed delta didn't change on its own.
        assert_eq!(time.fixed_delta(), fixed_delta);
        assert_eq!(time.fixed_delta_seconds(), fixed_delta.as_secs_f32());
        assert_eq!(time.fixed_delta_seconds_f64(), fixed_delta.as_secs_f64());

        // Confirm that the fixed time clock has advanced 10 steps worth of time.
        assert_eq!(time.fixed_elapsed_since_startup(), ten);
        assert_eq!(time.fixed_seconds_since_startup(), ten.as_secs_f32());
        assert_eq!(time.fixed_seconds_since_startup_f64(), ten.as_secs_f64());

        // Confirm that the fixed clock lags behind the normal clock by a specific amount.
        let diff = time.elapsed_since_startup() - time.fixed_elapsed_since_startup();
        let expected = time.first_update().unwrap() - time.startup() + accumulator.time();
        assert_eq!(diff, expected);
    }
}