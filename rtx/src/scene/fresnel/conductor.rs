use crate::core::vec3;
use crate::scene::fresnel;

pub struct Conductor {
    eta_i: vec3::Vec3,
    eta_t: vec3::Vec3,
    k: vec3::Vec3,
}

impl Conductor {
    pub fn new(eta_i: vec3::Vec3, eta_t: vec3::Vec3, k: vec3::Vec3) -> Conductor {
        return Conductor { eta_i, eta_t, k };
    }
}

impl fresnel::Fresnel for Conductor {
    fn evaluate(&self, dot_normal_wo: f32) -> vec3::Vec3 {
        let eta_i;
        let eta_t;
        let cos_normal_wo;
        if dot_normal_wo >= 0.0 {
            cos_normal_wo = dot_normal_wo;
            eta_i = self.eta_i;
            eta_t = self.eta_t;
        } else {
            cos_normal_wo = f32::abs(dot_normal_wo);
            eta_i = self.eta_t;
            eta_t = self.eta_i;
        }

        let cos_normal_wo_sq = vec3::Vec3::from(cos_normal_wo * cos_normal_wo);
        let sin_normal_wo_sq = vec3::Vec3::from(1.0) - cos_normal_wo_sq;

        let eta = eta_t / eta_i;
        let eta_sq = eta * eta;

        let eta_k = self.k / self.eta_i;
        let eta_k_sq = eta_k * eta_k;

        let m = eta_sq - eta_k_sq - sin_normal_wo_sq;
        let a_sq_plus_b_sq_sq = m * m + 4.0 * eta_sq * eta_k_sq;
        let a_sq_plus_b_sq = vec3::Vec3::sqrt(&a_sq_plus_b_sq_sq);
        let a_sq = 0.5 * (a_sq_plus_b_sq + eta_sq - eta_k_sq - sin_normal_wo_sq);
        let a = vec3::Vec3::sqrt(&a_sq);

        let t0 = a_sq_plus_b_sq + cos_normal_wo_sq;
        let t1 = 2.0 * a * cos_normal_wo;
        let r_perpendicular = (t0 - t1) / (t0 + t1);

        let t3 = cos_normal_wo_sq * a_sq_plus_b_sq + sin_normal_wo_sq * sin_normal_wo_sq;
        let t4 = 2.0 * a * cos_normal_wo * sin_normal_wo_sq;
        let r_parallel = r_perpendicular * (t3 - t4) / (t3 + t4);

        return 0.5 * (r_parallel * r_parallel + r_perpendicular * r_perpendicular);
    }
}
