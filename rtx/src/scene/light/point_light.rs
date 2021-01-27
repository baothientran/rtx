use crate::core::vec3;
use crate::scene::light;
use crate::scene::ray;
use crate::scene::sampler;
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
    fn sample_li(
        &self,
        _sampler: &mut dyn sampler::Sampler,
        world: &world::World,
        surface_point: &vec3::Vec3,
        wi: &mut vec3::Vec3,
    ) -> vec3::Vec3 {
        let direction = self.position - surface_point;
        let normalize_direction = direction.normalize().unwrap();

        let max_distance = vec3::Vec3::length(&direction);
        let ray = ray::Ray::new(*surface_point, normalize_direction);
        if world.is_intersect(&ray, max_distance) {
            return vec3::Vec3::from(0.0);
        }

        let distance_sq = vec3::Vec3::length_sq(&direction);
        let attenuation = f32::max(1.0 - distance_sq / (self.radius * self.radius), 0.0);

        *wi = normalize_direction;
        return attenuation * self.color;
    }
}
