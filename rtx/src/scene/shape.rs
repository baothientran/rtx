pub mod rectangle;
pub mod sphere;

use crate::core::mat4;
use crate::core::vec2;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::ray;
use crate::scene::material;
use std::rc;

#[derive(Debug)]
pub struct IntersectableShapeSurface<'a> {
    ray_time: f32,
    position: vec3::Vec3,
    normal: vec3::Vec3,
    dpdu: vec3::Vec3,
    dpdv: vec3::Vec3,
    object_to_world: &'a mat4::Mat4,
    normal_transform: &'a mat4::Mat4,
}

impl<'a> IntersectableShapeSurface<'a> {
    pub fn new(
        ray_time: f32,
        position: vec3::Vec3,
        normal: vec3::Vec3,
        dpdu: vec3::Vec3,
        dpdv: vec3::Vec3,
        object_to_world: &'a mat4::Mat4,
        normal_transform: &'a mat4::Mat4,
    ) -> IntersectableShapeSurface<'a> {
        return IntersectableShapeSurface {
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

pub trait IntersectableShape {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool;

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<IntersectableShapeSurface>;
}

#[derive(Debug)]
pub struct SampleShapeSurface {
    pub pdf: f32,
    pub surface_point: vec3::Vec3,
}

impl SampleShapeSurface {
    pub fn new(pdf: f32, surface_point: vec3::Vec3) -> SampleShapeSurface {
        return SampleShapeSurface { pdf, surface_point };
    }
}

pub trait SamplableShape {
    fn sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        surface_normal_ref: &vec3::Vec3,
    ) -> Option<SampleShapeSurface>;
}

pub struct RenderableShapeSurface<'a> {
    shape_surface: IntersectableShapeSurface<'a>,
    material: &'a dyn material::Material,
}

impl<'a> RenderableShapeSurface<'a> {
    pub fn new(
        shape_surface: IntersectableShapeSurface<'a>,
        material: &'a dyn material::Material,
    ) -> RenderableShapeSurface<'a> {
        return RenderableShapeSurface {
            shape_surface,
            material,
        };
    }

    pub fn shape_surface(&self) -> &IntersectableShapeSurface {
        return &self.shape_surface;
    }

    pub fn material(&self) -> &dyn material::Material {
        return self.material;
    }
}

pub struct RenderableShape {
    shape: rc::Rc<dyn IntersectableShape>,
    material: rc::Rc<dyn material::Material>,
}

impl RenderableShape {
    pub fn new(
        shape: rc::Rc<dyn IntersectableShape>,
        material: rc::Rc<dyn material::Material>,
    ) -> RenderableShape {
        return RenderableShape { shape, material };
    }

    pub fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        return self.shape.is_intersect(ray, max_distance);
    }

    pub fn intersect_ray(&self, ray: &ray::Ray) -> Option<RenderableShapeSurface> {
        return match self.shape.intersect_ray(ray) {
            Some(hit_record) => Some(RenderableShapeSurface::new(hit_record, self.material.as_ref())),
            None => None,
        };
    }
}
