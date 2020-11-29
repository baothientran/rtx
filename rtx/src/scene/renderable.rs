use crate::scene::ray;
use crate::core::vec3;

pub enum SurfaceInfo {
    None,
    Hit {
        ray_time: f32,
        position: vec3::Vec3,
        normal: vec3::Vec3,
    },
}

pub trait Renderable {
    fn intersect_ray(&self, ray: &ray::Ray, surface_info: &mut SurfaceInfo);
}
