pub mod point_light;

use crate::core::vec3;

pub trait Light {
    fn li(&self, surface_point: &vec3::Vec3, wi: &mut vec3::Vec3) -> vec3::Vec3;
}
