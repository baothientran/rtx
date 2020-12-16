use material::MaterialType;

use crate::core::math;
use crate::core::vec3;
use crate::scene::material;

pub struct Lambertian {
    kd: vec3::Vec3,
}

impl Lambertian {
    pub fn new(kd: vec3::Vec3) -> Lambertian {
        return Lambertian { kd };
    }
}

impl material::Material for Lambertian {
    fn has_types(&self, flags: u32) -> bool {
        return MaterialType::contain(material::MaterialType::Lambertian as u32, flags);
    }

    fn brdf(
        &self,
        _surface_point: &vec3::Vec3,
        _normal: &vec3::Vec3,
        _wo: &vec3::Vec3,
        _wi: &vec3::Vec3,
    ) -> vec3::Vec3 {
        return self.kd / math::PI_F32;
    }

    fn sample_brdf(
        &self,
        surface_point: &vec3::Vec3,
        normal: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &mut vec3::Vec3,
    ) -> vec3::Vec3 {
        return self.brdf(surface_point, normal, wo, wi);
    }
}
