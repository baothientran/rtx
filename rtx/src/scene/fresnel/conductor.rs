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
    fn evaluate(&self, cos_theta_i: f32) -> vec3::Vec3 {
        let eta_i;
        let eta_t;
        let mut cos_theta_i = cos_theta_i;
        if cos_theta_i >= 0.0 {
            eta_i = self.eta_i;
            eta_t = self.eta_t;
        } else {
            cos_theta_i = f32::abs(cos_theta_i);
            eta_i = self.eta_t;
            eta_t = self.eta_i;
        }

        let cos_theta_i_sq = vec3::Vec3::from(cos_theta_i * cos_theta_i);
        let sin_theta_i_sq = vec3::Vec3::from(1.0) - cos_theta_i_sq;

        let eta = eta_t / eta_i;
        let eta_sq = eta * eta;

        let eta_k = self.k / self.eta_i;
        let eta_k_sq = eta_k * eta_k;

        let m = eta_sq - eta_k_sq - sin_theta_i_sq;
        let a_sq_plus_b_sq_sq = m * m + 4.0 * eta_sq * eta_k_sq;
        let a_sq_plus_b_sq = vec3::Vec3::sqrt(&a_sq_plus_b_sq_sq);
        let a_sq = 0.5 * (a_sq_plus_b_sq + eta_sq - eta_k_sq - sin_theta_i_sq);
        let a = vec3::Vec3::sqrt(&a_sq);

        let t0 = a_sq_plus_b_sq + cos_theta_i_sq;
        let t1 = 2.0 * a * cos_theta_i;
        let r_perpendicular = (t0 - t1) / (t0 + t1);

        let t3 = cos_theta_i_sq * a_sq_plus_b_sq + sin_theta_i_sq * sin_theta_i_sq;
        let t4 = 2.0 * a * cos_theta_i * sin_theta_i_sq;
        let r_parallel = r_perpendicular * (t3 - t4) / (t3 + t4);

        return 0.5 * (r_parallel + r_perpendicular);
    }
}
