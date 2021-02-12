use crate::core::math;
use crate::core::vec3;
use crate::scene::fresnel::dielectrics;
use crate::scene::material;
use crate::scene::reflectance;
use crate::scene::reflectance::{reflection, refraction};
use std::rc;

pub struct Glass {
    reflectances: reflectance::ReflectanceCollection,
}

impl Glass {
    pub fn new(kt: vec3::Vec3, kr: vec3::Vec3, eta_i: f32, eta_t: f32) -> Glass {
        let mut reflectances = reflectance::ReflectanceCollection::new();
        if !vec3::Vec3::equal_epsilon(&kr, &vec3::Vec3::from(0.0), math::EPSILON_F32_6) {
            reflectances.add(Box::new(reflection::Reflection::new(
                kr,
                rc::Rc::new(dielectrics::Dielectrics::new(eta_i, eta_t)),
            )));
        }

        if !vec3::Vec3::equal_epsilon(&kt, &vec3::Vec3::from(0.0), math::EPSILON_F32_6) {
            reflectances.add(Box::new(refraction::Refraction::new(eta_i, eta_t, kt)));
        }

        return Glass { reflectances };
    }
}

impl material::Material for Glass {
    fn bxdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &vec3::Vec3,
    ) -> vec3::Vec3 {
        return self.reflectances.bxdf(normal, dpdu, wo, wi);
    }

    fn sample_bxdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &mut Option<vec3::Vec3>,
        flags: u32,
    ) -> Option<vec3::Vec3> {
        return self.reflectances.sample_bxdf(normal, dpdu, wo, wi, flags);
    }
}
