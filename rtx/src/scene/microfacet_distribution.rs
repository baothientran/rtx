pub mod beckmann_spizzichino;
pub mod trowbridge_reitz;

use crate::core::vec3;

pub trait MicrofacetDistribution {
    fn d(&self, wh: &vec3::Vec3) -> f32;
}
