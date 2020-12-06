pub mod lambertian;

use crate::core::vec3;

pub trait Material {
    fn le(&self, _surface_point: &vec3::Vec3, _wo: &vec3::Vec3) -> vec3::Vec3 {
        return vec3::Vec3::from(0.0);
    }

    fn brdf(&self, surface_point: &vec3::Vec3, wo: &vec3::Vec3, wi: &vec3::Vec3) -> vec3::Vec3;
}
