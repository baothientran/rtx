use crate::core::vec2;
use crate::scene::sampler;
use rand::prelude::*;
use rand::rngs;

pub struct RandomSampler {
    rng: rngs::ThreadRng,
}

impl RandomSampler {
    pub fn new() -> RandomSampler {
        return RandomSampler {
            rng: rand::thread_rng(),
        };
    }
}

impl sampler::Sampler for RandomSampler {
    fn get_1d(&mut self) -> f32 {
        return self.rng.gen_range(0.0, 1.0);
    }

    fn get_2d(&mut self) -> vec2::Vec2 {
        return vec2::Vec2::new(self.get_1d(), self.get_1d());
    }

    fn get_1d_array(&mut self, n: usize) -> Vec<f32> {
        let mut samples = Vec::with_capacity(n);
        for _ in 0..n {
            samples.push(self.get_1d());
        }

        return samples;
    }

    fn get_2d_array(&mut self, n: usize) -> Vec<vec2::Vec2> {
        let mut samples = Vec::with_capacity(n);
        for _ in 0..n {
            samples.push(self.get_2d());
        }

        return samples;
    }
}
