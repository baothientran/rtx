use crate::core::mat4;
use crate::core::math;
use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

pub struct Disk {
    inner_radius: f32,
    outer_radius: f32,
    object_to_world: mat4::Mat4,
    world_to_object: mat4::Mat4,
    normal_transform: mat4::Mat4,
}

impl Disk {
    pub fn new(object_to_world: mat4::Mat4, inner_radius: f32, outer_radius: f32) -> Disk {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();

        return Disk {
            inner_radius,
            outer_radius,
            object_to_world,
            world_to_object,
            normal_transform,
        };
    }
}

impl shape::IntersectableShape for Disk {
    fn is_intersect(&self, ray: &crate::scene::ray::Ray, max_distance: f32) -> bool {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        if local_ray.direction().z == 0.0 {
            return false;
        }

        let ray_time = -local_ray.origin().z / local_ray.direction().z;
        if ray_time <= 0.0 {
            return false;
        }

        let local_position = local_ray.calc_position(ray_time);
        let local_position_radius = local_position.length();
        return local_position_radius >= self.inner_radius
            && local_position_radius <= self.outer_radius
            && ray_time < max_distance;
    }

    fn intersect_ray(
        &self,
        ray: &crate::scene::ray::Ray,
    ) -> Option<shape::IntersectableShapeSurface> {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        if local_ray.direction().z == 0.0 {
            return None;
        }

        let ray_time = -local_ray.origin().z / local_ray.direction().z;
        if ray_time <= 0.0 {
            return None;
        }

        let local_position = local_ray.calc_position(ray_time);
        let local_position_radius = local_position.length();

        if local_position_radius < self.inner_radius {
            return None;
        }

        if local_position_radius > self.outer_radius {
            return None;
        }

        let local_dpdu =
            vec3::Vec3::new(-local_position.x, local_position.y, 0.0) * 2.0 * math::PI_F32;
        let local_dpdv = vec3::Vec3::new(local_position.x, local_position.y, 0.0)
            * (self.inner_radius - self.outer_radius)
            / local_position_radius;

        let mut normal = vec3::Vec3::new(0.0, 0.0, 1.0);
        if local_ray.direction().dot(&normal) > 0.0 {
            normal = -normal;
        }

        return Some(shape::IntersectableShapeSurface::new(
            ray_time,
            local_position,
            normal,
            local_dpdu,
            local_dpdv,
            &self.object_to_world,
            &self.normal_transform,
        ));
    }
}
