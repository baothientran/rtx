pub mod conductor;
pub mod dielectrics;

use crate::core::vec3;

pub trait Fresnel {
    fn evaluate(&self, dot_normal_wo: f32) -> vec3::Vec3;
}
