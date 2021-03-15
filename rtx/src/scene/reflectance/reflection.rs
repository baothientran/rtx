use crate::core::vec2;
use crate::core::vec3;
use crate::scene::fresnel;
use crate::scene::reflectance;
use std::rc;

pub struct Reflection {
    kr: vec3::Vec3,
    fresnel: rc::Rc<dyn fresnel::Fresnel>,
}

impl Reflection {
    pub fn new(kr: vec3::Vec3, fresnel: rc::Rc<dyn fresnel::Fresnel>) -> Reflection {
        return Reflection { kr, fresnel };
    }
}

impl reflectance::Reflectance for Reflection {
    fn has_types(&self, flags: u32) -> bool {
        return reflectance::ReflectanceType::contain(
            reflectance::ReflectanceType::Specular as u32
                | reflectance::ReflectanceType::Reflection as u32,
            flags,
        );
    }

    fn bxdf(&self, _shading_wo: &vec3::Vec3, _shading_wi: &vec3::Vec3) -> vec3::Vec3 {
        return vec3::Vec3::from(0.0);
    }

    fn sample_bxdf(
        &self,
        _sample: &vec2::Vec2,
        shading_wo: &vec3::Vec3,
    ) -> Option<reflectance::ShadingReflectanceRadiance> {
        let shading_wi = vec3::Vec3::new(-shading_wo.x, -shading_wo.y, shading_wo.z);
        let cos_theta_wi = shading_wi.z;
        return Some(reflectance::ShadingReflectanceRadiance {
            shading_wi,
            bxdf: self.fresnel.evaluate(cos_theta_wi) * self.kr / f32::abs(cos_theta_wi),
            pdf: 1.0,
        });
    }
}
