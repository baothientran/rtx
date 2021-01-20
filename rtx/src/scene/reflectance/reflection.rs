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
            reflectance::ReflectanceType::Reflection as u32,
            flags,
        );
    }

    fn bxdf(&self, _shading_wo: &vec3::Vec3, _shading_wi: &vec3::Vec3) -> vec3::Vec3 {
        return vec3::Vec3::from(0.0);
    }

    fn sample_bxdf(&self, shading_wo: &vec3::Vec3, shading_wi: &mut vec3::Vec3) -> vec3::Vec3 {
        *shading_wi = vec3::Vec3::new(-shading_wo.x, -shading_wo.y, shading_wo.z); // reflect against z-axis
        let cos_theta_wi = shading_wi.z;
        return self.fresnel.evaluate(cos_theta_wi) * self.kr / f32::abs(cos_theta_wi);
    }
}
