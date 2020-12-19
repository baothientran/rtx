pub mod conductor;
pub mod dielectrics;

use crate::core::vec3;

pub trait Fresnel {
    fn evaluate(&self, cos_theta_i: f32) -> vec3::Vec3;
}
