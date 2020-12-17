use crate::scene::material;

pub struct Refraction {

}

impl Refraction {

}

impl material::Material for Refraction {
    fn has_types(&self, flags: u32) -> bool {
        todo!()
    }

    fn brdf(
        &self,
        dot_normal_wo: f32,
        normal: &crate::core::vec3::Vec3,
        wo: &crate::core::vec3::Vec3,
        wi: &crate::core::vec3::Vec3,
    ) -> crate::core::vec3::Vec3 {
        todo!()
    }

    fn sample_brdf(
        &self,
        dot_normal_wo: f32,
        normal: &crate::core::vec3::Vec3,
        wo: &crate::core::vec3::Vec3,
        wi: &mut crate::core::vec3::Vec3,
    ) -> crate::core::vec3::Vec3 {
        todo!()
    }
}