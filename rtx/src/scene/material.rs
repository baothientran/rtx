pub mod lambertian;

use crate::core::vec3;
use crate::scene::ray;

pub enum ShadeInfo {
    Color(vec3::Vec3),
    Reflect(ray::Ray),
}

pub trait Material {
    fn shade(ray: &ray::Ray, position: &vec3::Vec3, normal: &vec3::Vec3, depth: u32) -> ShadeInfo;
}
