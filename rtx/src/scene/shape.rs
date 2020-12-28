pub mod plane;
pub mod sphere;

use crate::core::vec3;
use crate::scene::ray;

#[derive(Copy, Clone, Debug)]
pub struct ShapeSurface {
    ray_time: f32,
    position: vec3::Vec3,
    normal: vec3::Vec3,
    dpdu: vec3::Vec3,
    dpdv: vec3::Vec3,
}

impl ShapeSurface {
    pub fn new(
        ray_time: f32,
        position: vec3::Vec3,
        normal: vec3::Vec3,
        dpdu: vec3::Vec3,
        dpdv: vec3::Vec3,
    ) -> ShapeSurface {
        return ShapeSurface {
            ray_time,
            position,
            normal,
            dpdu,
            dpdv,
        };
    }

    pub fn ray_time(&self) -> f32 {
        return self.ray_time;
    }

    pub fn position(&self) -> &vec3::Vec3 {
        return &self.position;
    }

    pub fn normal(&self) -> &vec3::Vec3 {
        return &self.normal;
    }

    pub fn dpdu(&self) -> &vec3::Vec3 {
        return &self.dpdu;
    }

    pub fn dpdv(&self) -> &vec3::Vec3 {
        return &self.dpdv;
    }
}

pub trait Shape {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool;

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<ShapeSurface>;
}
