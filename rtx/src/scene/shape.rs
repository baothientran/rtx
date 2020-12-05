pub mod plane;
pub mod sphere;

use crate::core::vec3;
use crate::scene::ray;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub ray_time: f32,
    pub position: vec3::Vec3,
    pub normal: vec3::Vec3,
}

pub trait Shape {
    fn intersect_ray(&self, ray: &ray::Ray) -> Option<HitRecord>;
}
