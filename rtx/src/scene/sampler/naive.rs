use crate::core::vec2;
use crate::scene::sampler;
use rand::prelude::*;
use rand::rngs;

pub struct Naive {
    rng: rngs::ThreadRng,
}

impl Naive {
    pub fn new() -> Naive {
        return Naive {
            rng: rand::thread_rng(),
        };
    }
}

impl sampler::Sampler for Naive {
    fn sample_1d(&mut self) -> f32 {
        return self.rng.gen_range(0.0, 1.0);
    }

    fn sample_2d(&mut self) -> vec2::Vec2 {
        return vec2::Vec2::new(self.sample_1d(), self.sample_1d());
    }
}
