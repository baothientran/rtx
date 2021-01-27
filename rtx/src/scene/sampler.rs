pub mod naive;

use crate::core::vec2;

pub trait Sampler {
    fn sample_1d(&mut self) -> f32;

    fn sample_2d(&mut self) -> vec2::Vec2;
}
