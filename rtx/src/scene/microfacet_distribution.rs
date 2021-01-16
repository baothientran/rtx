pub mod beckmann_spizzichino;
pub mod trowbridge_reitz;

use crate::core::vec3;

pub trait MicrofacetDistribution {
    fn d(&self, shading_wh: &vec3::Vec3) -> f32;

    fn g(&self, shading_wo: &vec3::Vec3, shading_wi: &vec3::Vec3) -> f32 {
        return 1.0 / (1.0 + self.lambda(shading_wo) + self.lambda(shading_wi));
    }

    fn g1(&self, shading_w: &vec3::Vec3) -> f32 {
        return 1.0 / (1.0 + self.lambda(shading_w));
    }

    fn lambda(&self, shading_w: &vec3::Vec3) -> f32;
}
