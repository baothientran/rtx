use crate::core::math;
use crate::core::vec3;
use crate::scene::reflectance;

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

impl reflectance::Reflectance for OrenNayar {
    fn has_types(&self, flags: u32) -> bool {
        return reflectance::ReflectanceType::contain(
            reflectance::ReflectanceType::Microfacet as u32,
            flags,
        );
    }

    fn brdf(&self, shading_wo: &vec3::Vec3, shading_wi: &vec3::Vec3) -> vec3::Vec3 {
        // calc sin(alpha) and tan(beta)
        let cos_wo_normal = f32::abs(shading_wo.z);
        let cos_wi_normal = f32::abs(shading_wi.z);
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

        let wo_xy_dist = f32::sqrt(shading_wo.x * shading_wo.x + shading_wo.y * shading_wo.y);
        let sin_phi_o = shading_wo.y / wo_xy_dist;
        let cos_phi_o = shading_wo.x / wo_xy_dist;

        let wi_xy_dist = f32::sqrt(shading_wi.x * shading_wi.x + shading_wi.y * shading_wi.y);
        let sin_phi_i = shading_wi.y / wi_xy_dist;
        let cos_phi_i = shading_wi.x / wi_xy_dist;

        let cos_phi_diff = cos_phi_o * cos_phi_i + sin_phi_o * sin_phi_i;

        return self.kd / math::PI_F32
            * (self.a + self.b * f32::max(0.0, cos_phi_diff) * sin_alpha * tan_beta);
    }

    fn sample_brdf(&self, shading_wo: &vec3::Vec3, shading_wi: &mut vec3::Vec3) -> vec3::Vec3 {
        return self.brdf(shading_wo, shading_wi);
    }
}
