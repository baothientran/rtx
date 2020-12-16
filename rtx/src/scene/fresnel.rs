pub mod dielectrics;

use crate::core::vec3;

pub trait Fresnel {
    fn evaluate(&self, normal: &vec3::Vec3, wo: &vec3::Vec3) -> f32;
}
