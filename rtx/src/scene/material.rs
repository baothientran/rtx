pub mod glass;
pub mod matte;

use crate::core::vec3;
use crate::scene::reflectance;

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
        flags: u32,
    ) -> Option<reflectance::ReflectanceRadiance>;
}
