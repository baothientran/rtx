use crate::core::mat4;
use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::light;
use crate::scene::ray;
use crate::scene::sampler;
use crate::scene::world;

pub struct RectangleLight {
    object_to_world: mat4::Mat4,
    normal: vec3::Vec3,
    width: f32,
    height: f32,
    color: vec3::Vec3,
}

impl RectangleLight {
    pub fn new(
        object_to_world: mat4::Mat4,
        width: f32,
        height: f32,
        color: vec3::Vec3,
    ) -> RectangleLight {
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();
        let vec4_normal = normal_transform * vec4::Vec4::new(0.0, 0.0, 1.0, 0.0);
        let normal = vec4_normal.to_vec3().normalize().unwrap();

        return RectangleLight {
            object_to_world,
            color,
            normal,
            width,
            height,
        };
    }

    fn plane_to_world(&self, plane_point: vec2::Vec2) -> vec3::Vec3 {
        let local_point = vec3::Vec3::from_vec2(&plane_point, 0.0)
            - vec3::Vec3::new(self.width * 0.5, self.height * 0.5, 0.0);
        let world_point = self.object_to_world * vec4::Vec4::from_vec3(&local_point, 1.0);
        return world_point.to_vec3();
    }
}

impl light::Light for RectangleLight {
    fn sample_li(
        &self,
        sampler: &mut dyn sampler::Sampler,
        world: &world::World,
        surface_point: &vec3::Vec3,
        wi: &mut vec3::Vec3,
    ) -> vec3::Vec3 {
        let canonical_sampler = sampler.get_2d();
        let local_sample_point = vec2::Vec2::new(
            self.width * canonical_sampler.x,
            self.height * canonical_sampler.y,
        );
        let world_sample_point = self.plane_to_world(local_sample_point);

        let direction = world_sample_point - surface_point;
        let normalize_direction = direction.normalize().unwrap();
        let max_distance = direction.length();

        let ray = ray::Ray::new(*surface_point, normalize_direction);
        if world.is_intersect(&ray, max_distance) {
            return vec3::Vec3::from(0.0);
        }

        *wi = normalize_direction;
        let area = self.width * self.height;
        return area * self.color * f32::max(-self.normal.dot(&normalize_direction), 0.0)
            / (direction.length_sq() * math::PI_F32);
    }
}
