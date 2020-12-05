use crate::core::vec3;
use crate::scene::material;
use crate::scene::ray;

pub struct Lamertian {
    color: vec3::Vec3,
}

impl Lamertian {
    pub fn new(color: vec3::Vec3) -> Lamertian {
        return Lamertian { color };
    }
}

impl material::Material for Lamertian {
    fn shade(
        ray: &ray::Ray,
        position: &vec3::Vec3,
        normal: &vec3::Vec3,
        depth: u32,
    ) -> material::ShadeInfo {
        return material::ShadeInfo::Color(vec3::Vec3::from(0.0));
    }
}
