use crate::core::vec3;
use crate::scene::fresnel::dielectrics;
use crate::scene::fresnel::Fresnel;
use crate::scene::reflectance;

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

impl reflectance::Reflectance for Refraction {
    fn has_types(&self, flags: u32) -> bool {
        return reflectance::ReflectanceType::contain(
            reflectance::ReflectanceType::Specular as u32 |
            reflectance::ReflectanceType::Refraction as u32,
            flags,
        );
    }

    fn bxdf(
        &self,
        _shading_wo: &crate::core::vec3::Vec3,
        _shading_wi: &crate::core::vec3::Vec3,
    ) -> vec3::Vec3 {
        return vec3::Vec3::from(0.0);
    }

    fn sample_bxdf(
        &self,
        shading_wo: &vec3::Vec3,
    ) -> Option<reflectance::ShadingReflectanceRadiance> {
        let mut cos_normal_wo = shading_wo.z;
        let eta_i;
        let eta_t;
        let mut n = vec3::Vec3::new(0.0, 0.0, 1.0);
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
            return None;
        }

        let shading_wi = r * (-shading_wo) + (r * cos_normal_wo - f32::sqrt(cos_normal_wi_sq)) * n;

        // calculate bxdf of refraction
        let cos_normal_wi = shading_wi.z;
        r *= r;
        return Some(reflectance::ShadingReflectanceRadiance {
            shading_wi,
            bxdf: r * (vec3::Vec3::from(1.0) - self.fresnel.evaluate(cos_normal_wi)) * self.kt
                / f32::abs(cos_normal_wi),
        });
    }
}
