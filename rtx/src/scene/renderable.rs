use shape::ShapeSurface;

use crate::scene::reflectance;
use crate::scene::ray;
use crate::scene::shape;
use std::rc;

pub struct RenderableSurface<'a> {
    shape_surface: shape::ShapeSurface<'a>,
    material: &'a dyn reflectance::Reflectance,
}

impl<'a> RenderableSurface<'a> {
    pub fn new(
        shape_surface: shape::ShapeSurface<'a>,
        material: &'a dyn reflectance::Reflectance,
    ) -> RenderableSurface<'a> {
        return RenderableSurface {
            shape_surface,
            material,
        };
    }

    pub fn shape_surface(&self) -> &ShapeSurface {
        return &self.shape_surface;
    }

    pub fn material(&self) -> &dyn reflectance::Reflectance {
        return self.material;
    }
}

pub struct Renderable {
    shape: rc::Rc<dyn shape::Shape>,
    material: rc::Rc<dyn reflectance::Reflectance>,
}

impl Renderable {
    pub fn new(
        shape: rc::Rc<dyn shape::Shape>,
        material: rc::Rc<dyn reflectance::Reflectance>,
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
