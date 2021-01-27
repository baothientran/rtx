pub mod canonical;

use crate::core::vec2;

pub trait Sampler {
    fn sample_1D(&mut self) -> f32;

    fn sample_2D(&mut self) -> vec2::Vec2;
}
