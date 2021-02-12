pub mod glass;
pub mod matte;

use crate::core::vec3;

pub trait Material {
    fn bxdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &vec3::Vec3,
    ) -> vec3::Vec3;

    fn sample_bxdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &mut Option<vec3::Vec3>,
        flags: u32,
    ) -> Option<vec3::Vec3>;
}
