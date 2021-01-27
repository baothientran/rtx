pub mod naive_sampler;

use crate::core::vec2;

pub trait Sampler {
    fn get_1d(&mut self) -> f32;

    fn get_2d(&mut self) -> vec2::Vec2;
}
