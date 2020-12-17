use crate::core::vec3;
use crate::scene::fresnel;

pub struct Dielectrics {
    eta_i: f32,
    eta_t: f32,
}

impl Dielectrics {
    pub fn new(eta_i: f32, eta_t: f32) -> Dielectrics {
        return Dielectrics { eta_i, eta_t };
    }
}

impl fresnel::Fresnel for Dielectrics {
    fn evaluate(&self, dot_normal_wo: f32) -> vec3::Vec3 {
        let mut cos_theta_wo = dot_normal_wo;
        let eta_i;
        let eta_t;
        if cos_theta_wo > 0.0 {
            // wo enters the surface
            eta_i = self.eta_i;
            eta_t = self.eta_t;
        } else {
            // wo leaves the surface
            cos_theta_wo = f32::abs(cos_theta_wo);
            eta_i = self.eta_t;
            eta_t = self.eta_i;
        }

        let sin_theta_wo = f32::sqrt(1.0 - cos_theta_wo * cos_theta_wo);
        let sin_theta_wi = eta_i / eta_t * sin_theta_wo;
        if sin_theta_wi >= 1.0 {
            // total internal reflection
            return vec3::Vec3::from(1.0);
        }

        let cos_theta_wi = f32::sqrt(1.0 - sin_theta_wi * sin_theta_wi);
        let r_parallel = (eta_t * cos_theta_wo - eta_i * cos_theta_wi)
            / (eta_t * cos_theta_wo + eta_i * cos_theta_wi);
        let r_perpendicular = (eta_i * cos_theta_wo - eta_t * cos_theta_wi)
            / (eta_i * cos_theta_wo + eta_t * cos_theta_wi);

        return vec3::Vec3::from(0.5 * (r_parallel * r_parallel + r_perpendicular * r_perpendicular));
    }
}
