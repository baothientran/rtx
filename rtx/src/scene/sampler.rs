pub mod random_sampler;

use crate::core::vec2;

pub trait Sampler {
    fn get_1d(&mut self) -> f32;

    fn get_2d(&mut self) -> vec2::Vec2;

    fn get_1d_array(&mut self, n: usize) -> Vec<f32>;

    fn get_2d_array(&mut self, n: usize) -> Vec<vec2::Vec2>;
}
