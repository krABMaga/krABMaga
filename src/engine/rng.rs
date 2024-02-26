use rand::distributions::uniform::SampleRange;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::engine::bevy_ecs::prelude::Resource;

// TODO offer several constructors to allow the user to create scoped RNGs, such as an agent-based one, or a step-based one or both
#[derive(Resource, Clone)]
pub struct RNG {
    inner: ChaCha8Rng,
    float_range: Uniform<f32>,
}

impl RNG {
    pub fn new(seed: u64, stream: u64) -> Self {
        let mut chacha = ChaCha8Rng::seed_from_u64(seed);
        chacha.set_stream(stream);
        RNG {
            inner: chacha,
            float_range: Uniform::new(0.0f32, 1.0),
        }
    }

    pub fn set_stream(&mut self, stream: u64) {
        self.inner.set_stream(stream);
    }

    pub fn gen(&mut self) -> f32 {
        self.float_range.sample(&mut self.inner)
    }

    pub fn gen_range<T, D: SampleRange<T>>(&mut self, range: D) -> T {
        range.sample_single(&mut self.inner)
    }

    pub fn gen_bool(&mut self, prob: f64) -> bool {
        self.inner.gen_bool(prob)
    }
}
