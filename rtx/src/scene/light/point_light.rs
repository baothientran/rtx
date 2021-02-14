use crate::core::vec2;
use crate::core::vec3;
use crate::scene::light;
use crate::scene::ray;
use crate::scene::world;

pub struct PointLight {
    position: vec3::Vec3,
    radius: f32,
    color: vec3::Vec3,
}

impl PointLight {
    pub fn new(position: vec3::Vec3, color: vec3::Vec3, radius: f32) -> PointLight {
        return PointLight {
            position,
            radius,
            color,
        };
    }
}

impl light::Light for PointLight {
    fn num_samples(&self) -> u32 {
        return 1;
    }

    fn sample_li(
        &self,
        _sample: &vec2::Vec2,
        world: &world::World,
        surface_point: &vec3::Vec3,
        _surface_normal: &vec3::Vec3,
    ) -> Option<light::SampleLightRadiance> {
        let direction = self.position - surface_point;
        let normalize_direction = direction.normalize().unwrap();

        let max_distance = vec3::Vec3::length(&direction);
        let ray = ray::Ray::new(*surface_point, normalize_direction);
        if world.is_intersect(&ray, max_distance) {
            return None;
        }

        let distance_sq = vec3::Vec3::length_sq(&direction);
        let attenuation = f32::max(1.0 - distance_sq / (self.radius * self.radius), 0.0);

        return Some(light::SampleLightRadiance::new(
            normalize_direction,
            attenuation * self.color,
        ));
    }

    fn sample_li_no_shadow_check(
        &self,
        _sample: &vec2::Vec2,
        _world: &world::World,
        surface_point: &vec3::Vec3,
        _surface_normal: &vec3::Vec3,
    ) -> Option<light::SampleLightRadiance> {
        let direction = self.position - surface_point;
        let normalize_direction = direction.normalize().unwrap();
        let distance_sq = vec3::Vec3::length_sq(&direction);
        let attenuation = f32::max(1.0 - distance_sq / (self.radius * self.radius), 0.0);

        return Some(light::SampleLightRadiance::new(
            normalize_direction,
            attenuation * self.color,
        ));
    }
}
