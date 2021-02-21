use crate::core::vec3;
use crate::scene::fresnel;
use crate::scene::microfacet_distribution;
use crate::scene::reflectance;
use std::rc;

pub struct MicrofacetRefraction {
    kt: vec3::Vec3,
    distribution: rc::Rc<dyn microfacet_distribution::MicrofacetDistribution>,
    fresnel: rc::Rc<dyn fresnel::Fresnel>,
    eta_i: f32,
    eta_t: f32,
}

impl MicrofacetRefraction {
    pub fn new(
        kt: vec3::Vec3,
        distribution: rc::Rc<dyn microfacet_distribution::MicrofacetDistribution>,
        fresnel: rc::Rc<dyn fresnel::Fresnel>,
        eta_i: f32,
        eta_t: f32,
    ) -> MicrofacetRefraction {
        return MicrofacetRefraction {
            kt,
            distribution,
            fresnel,
            eta_i,
            eta_t,
        };
    }
}

impl reflectance::Reflectance for MicrofacetRefraction {
    fn has_types(&self, flags: u32) -> bool {
        return reflectance::ReflectanceType::contain(
            reflectance::ReflectanceType::Microfacet as u32
                | reflectance::ReflectanceType::Refraction as u32,
            flags,
        );
    }

    fn bxdf(&self, shading_wo: &vec3::Vec3, shading_wi: &vec3::Vec3) -> vec3::Vec3 {
        if shading_wi.z == 0.0 || shading_wo.z == 0.0 {
            return vec3::Vec3::from(0.0);
        }

        if shading_wi.z * shading_wo.z > 0.0 {
            return vec3::Vec3::from(0.0);
        }

        let eta_i;
        let eta_t;
        if shading_wo.z > 0.0 {
            eta_i = self.eta_i;
            eta_t = self.eta_t;
        } else {
            eta_i = self.eta_t;
            eta_t = self.eta_i;
        }

        let mut shading_wh = -eta_i * shading_wi - eta_t * shading_wo;
        if shading_wh.length_sq() == 0.0 {
            return vec3::Vec3::from(0.0);
        }

        shading_wh = shading_wh.normalize().unwrap();
        if shading_wh.z < 0.0 {
            shading_wh = -shading_wh;
        }

        let f = vec3::Vec3::from(1.0) - self.fresnel.evaluate(shading_wh.dot(shading_wi));
        let d = self.distribution.d(&shading_wh);
        let g = self.distribution.g(shading_wo, shading_wi);
        let wh_dot_wi = shading_wh.dot(shading_wi);
        let wh_dot_wo = shading_wh.dot(shading_wo);
        let s = eta_i * wh_dot_wi + eta_t * wh_dot_wo;
        return self.kt
            * self.eta_t
            * self.eta_t
            * f
            * d
            * g
            * f32::abs(wh_dot_wi)
            * f32::abs(wh_dot_wo)
            / (f32::abs(shading_wi.z) * f32::abs(shading_wo.z) * s * s);
    }

    fn sample_bxdf(
        &self,
        _shading_wo: &vec3::Vec3,
    ) -> Option<reflectance::ShadingReflectanceRadiance> {
        todo!()
    }
}
