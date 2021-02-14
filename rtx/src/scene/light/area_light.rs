use crate::core::vec2;
use crate::core::vec3;
use crate::scene::light;
use crate::scene::ray;
use crate::scene::shape;
use crate::scene::world;

pub struct AreaLight {
    color: vec3::Vec3,
    shape: Box<dyn shape::SamplableShape>,
    n_samples: u32,
}

impl AreaLight {
    pub fn new(color: vec3::Vec3, shape: Box<dyn shape::SamplableShape>, n_samples: u32) -> AreaLight {
        return AreaLight {
            color,
            shape,
            n_samples,
        };
    }
}

impl light::Light for AreaLight {
    fn num_samples(&self) -> u32 {
        return self.n_samples;
    }

    fn sample_li(
        &self,
        sample: &vec2::Vec2,
        world: &world::World,
        surface_point: &vec3::Vec3,
        surface_normal: &vec3::Vec3,
    ) -> Option<light::SampleLightRadiance> {
        let maybe_sample_shape_surface = self
            .shape
            .sample_surface(sample, &surface_point, &surface_normal);

        if maybe_sample_shape_surface.is_none() {
            return None;
        }

        let sample_shape_surface = maybe_sample_shape_surface.unwrap();
        let direction = sample_shape_surface.surface_point - surface_point;
        let normalize_direction = direction.normalize().unwrap();
        let ray = ray::Ray::new(*surface_point, normalize_direction);
        let max_distance = direction.length();
        if world.is_intersect(&ray, max_distance) {
            return None;
        }

        return Some(light::SampleLightRadiance::new(
            normalize_direction,
            self.color / sample_shape_surface.pdf,
        ));
    }

    fn sample_li_no_shadow_check(
        &self,
        sample: &vec2::Vec2,
        _world: &world::World,
        surface_point: &vec3::Vec3,
        surface_normal: &vec3::Vec3,
    ) -> Option<light::SampleLightRadiance> {
        let maybe_sample_shape_surface = self
            .shape
            .sample_surface(sample, &surface_point, &surface_normal);

        if maybe_sample_shape_surface.is_none() {
            return None;
        }

        let sample_shape_surface = maybe_sample_shape_surface.unwrap();
        let direction = sample_shape_surface.surface_point - surface_point;
        let normalize_direction = direction.normalize().unwrap();
        return Some(light::SampleLightRadiance::new(
            normalize_direction,
            self.color / sample_shape_surface.pdf,
        ));
    }
}
