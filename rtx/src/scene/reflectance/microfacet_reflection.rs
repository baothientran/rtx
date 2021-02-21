use std::u32;

use crate::core::vec3;
use crate::scene::fresnel;
use crate::scene::microfacet_distribution;
use crate::scene::reflectance;
use std::rc;

pub struct MicrofacetReflection {
    ks: vec3::Vec3,
    distribution: rc::Rc<dyn microfacet_distribution::MicrofacetDistribution>,
    fresnel: rc::Rc<dyn fresnel::Fresnel>,
}

impl MicrofacetReflection {
    pub fn new(
        ks: vec3::Vec3,
        distribution: rc::Rc<dyn microfacet_distribution::MicrofacetDistribution>,
        fresnel: rc::Rc<dyn fresnel::Fresnel>,
    ) -> MicrofacetReflection {
        return MicrofacetReflection {
            ks,
            distribution,
            fresnel,
        };
    }
}

impl reflectance::Reflectance for MicrofacetReflection {
    fn has_types(&self, flags: u32) -> bool {
        return reflectance::ReflectanceType::contain(
            reflectance::ReflectanceType::Microfacet as u32
                | reflectance::ReflectanceType::Reflection as u32,
            flags,
        );
    }

    fn bxdf(&self, shading_wo: &vec3::Vec3, shading_wi: &vec3::Vec3) -> vec3::Vec3 {
        let cos_theta_o = f32::abs(shading_wo.z);
        let cos_theta_i = f32::abs(shading_wi.z);
        if cos_theta_o == 0.0 || cos_theta_i == 0.0 {
            return vec3::Vec3::from(0.0);
        }

        let mut shading_wh = shading_wo + shading_wi;
        if shading_wh.x == 0.0 && shading_wh.y == 0.0 && shading_wh.z == 0.0 {
            return vec3::Vec3::from(0.0);
        }

        shading_wh = shading_wh.normalize().unwrap();
        let d = self.distribution.d(&shading_wh);
        let g = self.distribution.g(shading_wo, shading_wi);
        let f = self.fresnel.evaluate(shading_wh.dot(shading_wi));
        return self.ks * d * g * f / (4.0 * cos_theta_o * cos_theta_i);
    }

    fn sample_bxdf(
        &self,
        _shading_wo: &vec3::Vec3,
    ) -> Option<reflectance::ShadingReflectanceRadiance> {
        todo!();
    }
}
