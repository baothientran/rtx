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
    fn evaluate(&self, cos_theta_i: f32) -> vec3::Vec3 {
        let mut cos_theta_i = cos_theta_i;
        let eta_i;
        let eta_t;
        if cos_theta_i > 0.0 {
            // wo enters the surface
            eta_i = self.eta_i;
            eta_t = self.eta_t;
        } else {
            // wo leaves the surface
            cos_theta_i = f32::abs(cos_theta_i);
            eta_i = self.eta_t;
            eta_t = self.eta_i;
        }

        let sin_theta_i = f32::sqrt(1.0 - cos_theta_i * cos_theta_i);
        let sin_theta_t = eta_i / eta_t * sin_theta_i;
        if sin_theta_t >= 1.0 {
            // total internal reflection
            return vec3::Vec3::from(1.0);
        }

        let cos_theta_t = f32::sqrt(1.0 - sin_theta_t * sin_theta_t);
        let r_parallel = (eta_t * cos_theta_i - eta_i * cos_theta_t)
            / (eta_t * cos_theta_i + eta_i * cos_theta_t);
        let r_perpendicular = (eta_i * cos_theta_i - eta_t * cos_theta_t)
            / (eta_i * cos_theta_i + eta_t * cos_theta_t);

        return vec3::Vec3::from(
            0.5 * (r_parallel * r_parallel + r_perpendicular * r_perpendicular),
        );
    }
}
