use crate::core::vec3;
use crate::core::math;
use crate::scene::material;

pub struct Lambertian {
    color: vec3::Vec3,
}

impl Lambertian {
    pub fn new(color: vec3::Vec3) -> Lambertian {
        return Lambertian { color };
    }
}

impl material::Material for Lambertian {
    fn brdf(&self, _surface_point: &vec3::Vec3, _wo: &vec3::Vec3, _wi: &vec3::Vec3) -> vec3::Vec3 {
        return self.color / math::PI_F32;
    }
}
