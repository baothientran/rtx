use crate::core::vec2;
use crate::scene::sampler;
use rand::prelude::*;
use rand::rngs;

pub struct NaiveSampler {
    rng: rngs::ThreadRng,
}

impl NaiveSampler {
    pub fn new() -> NaiveSampler {
        return NaiveSampler {
            rng: rand::thread_rng(),
        };
    }
}

impl sampler::Sampler for NaiveSampler {
    fn get_1d(&mut self) -> f32 {
        return self.rng.gen_range(0.0, 1.0);
    }

    fn get_2d(&mut self) -> vec2::Vec2 {
        return vec2::Vec2::new(self.get_1d(), self.get_1d());
    }
}
