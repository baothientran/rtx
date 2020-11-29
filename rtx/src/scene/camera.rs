use crate::scene::ray;

pub trait Camera {
    fn create_ray(&self, x: f32, y: f32) -> ray::Ray;
}
