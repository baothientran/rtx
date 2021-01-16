use crate::core::vec3;
use crate::core::math;
use crate::scene::microfacet_distribution;

pub struct BeckmannSpizzichino {
    alpha_x: f32,
    alpha_y: f32
}

impl BeckmannSpizzichino {
    pub fn new(alpha_x: f32, alpha_y: f32) -> BeckmannSpizzichino {
        return BeckmannSpizzichino { alpha_x, alpha_y };
    }
}

impl microfacet_distribution::MicrofacetDistribution for BeckmannSpizzichino {
    fn d(&self, shading_wh: &vec3::Vec3) -> f32 {
        let cos_theta_sq = shading_wh.z * shading_wh.z;
        let tan_theta_sq = (1.0 - cos_theta_sq) / cos_theta_sq; 
        if f32::is_infinite(tan_theta_sq) {
            return 0.0;
        }

        let x_sq = shading_wh.x * shading_wh.x;
        let y_sq = shading_wh.y * shading_wh.y;
        let cos_phi_sq = x_sq / (x_sq + y_sq);
        let sin_phi_sq = 1.0 - cos_phi_sq;
        let a = cos_phi_sq / (self.alpha_x * self.alpha_x) + sin_phi_sq / (self.alpha_y * self.alpha_y); 

        let exp = f32::exp(-tan_theta_sq * a);
        return exp / (math::PI_F32 * self.alpha_x * self.alpha_y * cos_theta_sq * cos_theta_sq);
    }

    fn lambda(&self, shading_w: &vec3::Vec3) -> f32 {
        let cos_theta_sq = shading_w.z * shading_w.z;
        let tan_theta_sq = (1.0 - cos_theta_sq) / cos_theta_sq; 
        if f32::is_infinite(tan_theta_sq) {
            return 0.0;
        }

        let tan_theta = f32::sqrt(tan_theta_sq);
        let x_sq = shading_w.x * shading_w.x;
        let y_sq = shading_w.y * shading_w.y;
        let cos_phi_sq = x_sq / (x_sq + y_sq);
        let sin_phi_sq = 1.0 - cos_phi_sq;
        let a = 1.0 / (f32::sqrt(cos_phi_sq * self.alpha_x * self.alpha_x + sin_phi_sq * self.alpha_y * self.alpha_y) * tan_theta); 
        if a < 1.6 {
            let a_sq = a * a;
            return (1.0 - 1.259 * a + 0.396 * a_sq) / (3.535 * a + 2.181 * a_sq);
        }

        return 0.0;
    }
}
