use crate::core::math;
use crate::core::vec3;
use crate::scene::reflectance;

pub struct Lambertian {
    kd: vec3::Vec3,
}

impl Lambertian {
    pub fn new(kd: vec3::Vec3) -> Lambertian {
        return Lambertian { kd };
    }
}

impl reflectance::Reflectance for Lambertian {
    fn has_types(&self, flags: u32) -> bool {
        return reflectance::ReflectanceType::contain(
            reflectance::ReflectanceType::Lambertian as u32,
            flags,
        );
    }

    fn brdf(&self, _wo: &vec3::Vec3, _wi: &vec3::Vec3) -> vec3::Vec3 {
        return self.kd / math::PI_F32;
    }

    fn sample_brdf(&self, wo: &vec3::Vec3, wi: &mut vec3::Vec3) -> vec3::Vec3 {
        return self.brdf(wo, wi);
    }
}
