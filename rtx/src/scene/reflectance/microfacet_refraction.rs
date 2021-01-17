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

    fn brdf(&self, shading_wo: &vec3::Vec3, shading_wi: &vec3::Vec3) -> vec3::Vec3 {
        todo!()
    }

    fn sample_brdf(&self, _shading_wo: &vec3::Vec3, _shading_wi: &mut vec3::Vec3) -> vec3::Vec3 {
        todo!()
    }
}
