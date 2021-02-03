use crate::core::vec3;
use crate::scene::light;
use crate::scene::ray;
use crate::scene::sampler;
use crate::scene::world;
use crate::scene::shape;

pub struct AreaLight {
    color: vec3::Vec3,
    shape: Box<dyn shape::Shape>
}

impl AreaLight {
    pub fn new(color: vec3::Vec3, shape: Box<dyn shape::Shape>) -> AreaLight {
        return AreaLight { color, shape };
    }
}

impl light::Light for AreaLight {
    fn sample_li(
        &self,
        sampler: &mut dyn sampler::Sampler,
        world: &world::World,
        surface_point: &vec3::Vec3,
        wi: &mut vec3::Vec3,
    ) -> vec3::Vec3 {
        let uniform_sample = sampler.get_2d();
        let mut sample_surface = vec3::Vec3::from(0.0);
        let pdf = self.shape.pdf(&uniform_sample, &surface_point, &mut sample_surface);

        let direction = sample_surface - surface_point;
        let normalize_direction = direction.normalize().unwrap();
        let ray = ray::Ray::new(*surface_point, normalize_direction);
        let max_distance = direction.length();
        if world.is_intersect(&ray, max_distance) {
            return vec3::Vec3::from(0.0);
        }

        *wi = normalize_direction;
        return self.color / pdf;
    }
}
