use crate::core::vec3;
use crate::scene::material;
use crate::scene::ray;
use crate::scene::shape;
use std::rc;

pub struct RenderableSurface<'a> {
    shape_surface: shape::ShapeSurface,
    material: &'a dyn material::Material,
}

impl RenderableSurface<'_> {
    pub fn new(
        shape_surface: shape::ShapeSurface,
        material: &dyn material::Material,
    ) -> RenderableSurface {
        return RenderableSurface {
            shape_surface,
            material,
        };
    }

    pub fn ray_time(&self) -> f32 {
        return self.shape_surface.ray_time();
    }

    pub fn position(&self) -> &vec3::Vec3 {
        return self.shape_surface.position();
    }

    pub fn normal(&self) -> &vec3::Vec3 {
        return self.shape_surface.normal();
    }

    pub fn material(&self) -> &dyn material::Material {
        return self.material;
    }
}

pub struct Renderable {
    shape: rc::Rc<dyn shape::Shape>,
    material: rc::Rc<dyn material::Material>,
}

impl Renderable {
    pub fn new(
        shape: rc::Rc<dyn shape::Shape>,
        material: rc::Rc<dyn material::Material>,
    ) -> Renderable {
        return Renderable { shape, material };
    }

    pub fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        return self.shape.is_intersect(ray, max_distance);
    }

    pub fn intersect_ray(&self, ray: &ray::Ray) -> Option<RenderableSurface> {
        return match self.shape.intersect_ray(ray) {
            Some(hit_record) => Some(RenderableSurface::new(hit_record, self.material.as_ref())),
            None => None,
        };
    }
}
