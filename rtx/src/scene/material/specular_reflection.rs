use crate::core::vec3;
use crate::scene::fresnel;
use crate::scene::material;
use std::rc;

pub struct SpecularReflection {
    kr: vec3::Vec3,
    fresnel: rc::Rc<dyn fresnel::Fresnel>,
}

impl SpecularReflection {
    pub fn new(kr: vec3::Vec3, fresnel: rc::Rc<dyn fresnel::Fresnel>) -> SpecularReflection {
        return SpecularReflection { kr, fresnel };
    }
}

impl material::Material for SpecularReflection {
    fn has_types(&self, flags: u32) -> bool {
        return material::MaterialType::contain(material::MaterialType::Reflection as u32, flags);
    }

    fn brdf(
        &self,
        _normal: &vec3::Vec3,
        _wo: &vec3::Vec3,
        _wi: &vec3::Vec3,
    ) -> vec3::Vec3 {
        return vec3::Vec3::from(0.0);
    }

    fn sample_brdf(
        &self,
        normal: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &mut vec3::Vec3,
    ) -> vec3::Vec3 {
        *wi = vec3::Vec3::reflect(wo, normal);
        let cos_theta_wi = vec3::Vec3::dot(normal, wi);
        return self.fresnel.evaluate(normal, wo) * self.kr / f32::abs(cos_theta_wi);
    }
}
