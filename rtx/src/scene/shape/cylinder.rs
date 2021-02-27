use crate::core::vec2;
use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

pub struct Cylinder {}

impl Cylinder {
    pub fn new() -> Cylinder {
        return Cylinder {};
    }
}

impl shape::IntersectableShape for Cylinder {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        todo!()
    }

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::IntersectableShapeSurface> {
        todo!()
    }
}

impl shape::SamplableShape for Cylinder {
    fn sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        todo!()
    }
}
