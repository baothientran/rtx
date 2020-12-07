pub mod lambertian;

use crate::core::vec3;

pub trait Material {
    fn brdf(&self, surface_point: &vec3::Vec3, wo: &vec3::Vec3, wi: &vec3::Vec3) -> vec3::Vec3;
}
