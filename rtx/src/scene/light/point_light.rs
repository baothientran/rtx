use crate::core::vec3;
use crate::scene::light;

pub struct PointLight {
    position: vec3::Vec3,
    radius: f32,
    color: vec3::Vec3,
}

impl PointLight {
    pub fn new(position: vec3::Vec3, color: vec3::Vec3, radius: f32) -> PointLight {
        return PointLight {
            position,
            radius,
            color,
        };
    }
}

impl light::Light for PointLight {
    fn li(&self, surface_point: &vec3::Vec3, wi: &mut vec3::Vec3) -> vec3::Vec3 { 
        let direction = self.position - *surface_point;
        let distance = vec3::Vec3::length(&direction);

        *wi = vec3::Vec3::normalize(&direction).unwrap();
        return self.color;
    }
}