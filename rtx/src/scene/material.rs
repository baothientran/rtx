pub mod matte;

use crate::core::vec3;

pub trait Material {
    fn brdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &vec3::Vec3,
    ) -> vec3::Vec3;

    fn sample_brdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &mut vec3::Vec3,
        flags: u32,
    ) -> vec3::Vec3;
}
