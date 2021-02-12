use crate::core::vec2;
use crate::core::vec3;
use crate::scene::light;
use crate::scene::ray;
use crate::scene::shape;
use crate::scene::world;

pub struct AreaLight {
    color: vec3::Vec3,
    shape: Box<dyn shape::Shape>,
    n_samples: u32,
}

impl AreaLight {
    pub fn new(color: vec3::Vec3, shape: Box<dyn shape::Shape>, n_samples: u32) -> AreaLight {
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
        wi: &mut vec3::Vec3,
    ) -> vec3::Vec3 {
        let mut sample_surface = None;
        let pdf = self
            .shape
            .pdf(sample, &surface_point, &surface_normal, &mut sample_surface);
        
        if pdf == None {
            return vec3::Vec3::from(0.0);
        }

        let direction = sample_surface.unwrap() - surface_point;
        let normalize_direction = direction.normalize().unwrap();
        let ray = ray::Ray::new(*surface_point, normalize_direction);
        let max_distance = direction.length();
        if world.is_intersect(&ray, max_distance) {
            return vec3::Vec3::from(0.0);
        }

        *wi = normalize_direction;
        return self.color / pdf.unwrap();
    }
}
