use material::MaterialType;

use crate::core::math;
use crate::core::vec3;
use crate::scene::material;

pub struct OrenNayar {
    kd: vec3::Vec3,
    a: f32,
    b: f32,
}

impl OrenNayar {
    pub fn new(kd: vec3::Vec3, sigma: f32) -> OrenNayar {
        let sigma_sq = sigma * sigma;
        let a = 1.0 - 0.5 * sigma_sq / (sigma_sq + 0.33);
        let b = 0.45 * sigma_sq / (sigma_sq + 0.09);
        return OrenNayar { kd, a, b };
    }
}

impl material::Material for OrenNayar {
    fn has_types(&self, flags: u32) -> bool {
        return MaterialType::contain(MaterialType::Microfacet as u32, flags);
    }

    fn brdf(&self, normal: &vec3::Vec3, wo: &vec3::Vec3, wi: &vec3::Vec3) -> vec3::Vec3 {
        // determine if wo enters or leaves surface
        let mut n = *normal;
        let mut cos_wo_normal = vec3::Vec3::dot(normal, wo);
        if cos_wo_normal < 0.0 {
            cos_wo_normal = -cos_wo_normal;
            n = -n;
        }

        // calc sin(alpha) and tan(beta)
        let cos_wi_normal = vec3::Vec3::dot(&n, wi);
        let sin_alpha;
        let tan_beta;
        if cos_wo_normal > cos_wi_normal {
            sin_alpha = f32::sqrt(1.0 - cos_wi_normal * cos_wi_normal);
            let sin_beta = f32::sqrt(1.0 - cos_wo_normal * cos_wo_normal);
            tan_beta = sin_beta / cos_wo_normal;
        } else {
            sin_alpha = f32::sqrt(1.0 - cos_wo_normal * cos_wo_normal);
            let sin_beta = f32::sqrt(1.0 - cos_wi_normal * cos_wi_normal);
            tan_beta = sin_beta / cos_wi_normal;
        }

        // TODO: calculate cos(phi_o - phi_i)
        let cos_phi_diff = 0.0;

        return self.kd / math::PI_F32
            * (self.a + self.b * f32::max(0.0, cos_phi_diff) * sin_alpha * tan_beta);
    }

    fn sample_brdf(&self, normal: &vec3::Vec3, wo: &vec3::Vec3, wi: &mut vec3::Vec3) -> vec3::Vec3 {
        return self.brdf(normal, wo, wi);
    }
}
