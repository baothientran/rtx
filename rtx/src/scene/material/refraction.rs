use crate::core::vec3;
use crate::scene::fresnel::dielectrics;
use crate::scene::fresnel::Fresnel;
use crate::scene::material;

pub struct Refraction {
    eta_i: f32,
    eta_t: f32,
    kt: vec3::Vec3,
    fresnel: dielectrics::Dielectrics,
}

impl Refraction {
    pub fn new(eta_i: f32, eta_t: f32, kt: vec3::Vec3) -> Refraction {
        return Refraction {
            eta_i,
            eta_t,
            kt,
            fresnel: dielectrics::Dielectrics::new(eta_i, eta_t),
        };
    }
}

impl material::Material for Refraction {
    fn has_types(&self, flags: u32) -> bool {
        return material::MaterialType::contain(material::MaterialType::Refraction as u32, flags);
    }

    fn brdf(
        &self,
        _normal: &crate::core::vec3::Vec3,
        _wo: &crate::core::vec3::Vec3,
        _wi: &crate::core::vec3::Vec3,
    ) -> vec3::Vec3 {
        return vec3::Vec3::from(0.0);
    }

    fn sample_brdf(&self, normal: &vec3::Vec3, wo: &vec3::Vec3, wi: &mut vec3::Vec3) -> vec3::Vec3 {
        let mut cos_normal_wo = vec3::Vec3::dot(normal, wo);
        let mut n = *normal;
        let eta_i;
        let eta_t;
        if cos_normal_wo > 0.0 {
            eta_i = self.eta_i;
            eta_t = self.eta_t;
        } else {
            cos_normal_wo = f32::abs(cos_normal_wo);
            n = -n;
            eta_i = self.eta_t;
            eta_t = self.eta_i;
        }

        // calculate refraction wi
        let mut r = eta_i / eta_t;
        let cos_normal_wi_sq = 1.0 + r * r * (cos_normal_wo * cos_normal_wo - 1.0);

        // internal reflection occurs
        if cos_normal_wi_sq < 0.0 {
            return vec3::Vec3::from(0.0);
        }

        *wi = r * (-wo) + (r * cos_normal_wo - f32::sqrt(cos_normal_wi_sq)) * n;

        // calculate brdf of refraction
        let cos_normal_wi = vec3::Vec3::dot(normal, wi);
        r *= r;
        return r * (vec3::Vec3::from(1.0) - self.fresnel.evaluate(cos_normal_wi)) * self.kt
            / f32::abs(cos_normal_wi);
    }
}
