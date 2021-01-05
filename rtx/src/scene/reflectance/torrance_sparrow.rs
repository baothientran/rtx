use crate::core::vec3;
use crate::scene::reflectance;

pub struct TorranceSparrow {

}

impl reflectance::Reflectance for TorranceSparrow {
    fn has_types(&self, flags: u32) -> bool {
        todo!()
    }

    fn brdf(&self, shading_wo: &vec3::Vec3, shading_wi: &vec3::Vec3) -> vec3::Vec3 {
        todo!()
    }

    fn sample_brdf(&self, shading_wo: &vec3::Vec3, shading_wi: &mut vec3::Vec3) -> vec3::Vec3 {
        todo!()
    }
}