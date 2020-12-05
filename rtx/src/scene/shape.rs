pub mod plane;
pub mod sphere;

use crate::scene::ray;

pub trait Shape {
    fn intersect_ray(&self, ray: &ray::Ray) -> Option<f32>;
}
