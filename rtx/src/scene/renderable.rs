use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

pub struct RenderableSurface {
    shape_surface: shape::ShapeSurface,
}

impl RenderableSurface {
    pub fn new(shape_surface: shape::ShapeSurface) -> RenderableSurface {
        return RenderableSurface { shape_surface };
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
}

pub struct Renderable {
    shape: Box<dyn shape::Shape>,
}

impl Renderable {
    pub fn new(shape: Box<dyn shape::Shape>) -> Renderable {
        return Renderable { shape };
    }

    pub fn intersect_ray(&self, ray: &ray::Ray) -> Option<RenderableSurface> {
        return match self.shape.intersect_ray(ray) {
            Some(hit_record) => Some(RenderableSurface::new(hit_record)),
            None => None,
        };
    }
}
