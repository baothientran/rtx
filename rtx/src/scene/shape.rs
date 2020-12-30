pub mod plane;
pub mod sphere;

use crate::core::mat4;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::ray;

#[derive(Copy, Clone, Debug)]
pub struct ShapeSurface<'a> {
    ray_time: f32,
    position: vec3::Vec3,
    normal: vec3::Vec3,
    dpdu: vec3::Vec3,
    dpdv: vec3::Vec3,
    object_to_world: &'a mat4::Mat4,
    normal_transform: &'a mat4::Mat4,
}

impl<'a> ShapeSurface<'a> {
    pub fn new (
        ray_time: f32,
        position: vec3::Vec3,
        normal: vec3::Vec3,
        dpdu: vec3::Vec3,
        dpdv: vec3::Vec3,
        object_to_world: &'a mat4::Mat4,
        normal_transform: &'a mat4::Mat4,
    ) -> ShapeSurface<'a> {
        return ShapeSurface {
            ray_time,
            position,
            normal,
            dpdu,
            dpdv,
            object_to_world,
            normal_transform,
        };
    }

    pub fn ray_time(&self) -> f32 {
        return self.ray_time;
    }

    pub fn calc_world_position(&self) -> vec3::Vec3 {
        return vec4::Vec4::to_vec3(
            &(self.object_to_world * vec4::Vec4::from_vec3(&self.position, 1.0)),
        );
    }

    pub fn calc_world_normal(&self) -> vec3::Vec3 {
        let normal = vec4::Vec4::to_vec3(
            &(self.normal_transform * vec4::Vec4::from_vec3(&self.normal, 0.0)),
        );
        return vec3::Vec3::normalize(&normal).unwrap();
    }

    pub fn calc_world_dpdu(&self) -> vec3::Vec3 {
        return vec4::Vec4::to_vec3(
            &(self.object_to_world * vec4::Vec4::from_vec3(&self.dpdu, 0.0)),
        );
    }

    pub fn calc_world_dpdv(&self) -> vec3::Vec3 {
        return vec4::Vec4::to_vec3(
            &(self.object_to_world * vec4::Vec4::from_vec3(&self.dpdv, 0.0)),
        );
    }
}

pub trait Shape {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool;

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<ShapeSurface>;
}
