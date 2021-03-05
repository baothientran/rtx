use crate::core::mat4;
use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

pub struct Cylinder {
    object_to_world: mat4::Mat4,
    world_to_object: mat4::Mat4,
    normal_transform: mat4::Mat4,
    local_radius: f32,
    local_z_min: f32,
    local_z_max: f32,
}

impl Cylinder {
    pub fn new(
        object_to_world: mat4::Mat4,
        local_radius: f32,
        local_z_min: f32,
        local_z_max: f32,
    ) -> Cylinder {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();

        return Cylinder {
            object_to_world,
            world_to_object,
            normal_transform,
            local_radius,
            local_z_min,
            local_z_max,
        };
    }
}

impl shape::IntersectableShape for Cylinder {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let o = local_ray.origin();
        let d = local_ray.direction();
        let a = d.x * d.x + d.y * d.y;
        let b = 2.0 * (d.x * o.x + d.y * o.y);
        let c = o.x * o.x + o.y * o.y - self.local_radius * self.local_radius;

        let mut ray_time;
        if a == 0.0 {
            ray_time = -c / b;
        } else {
            let discriminant = b * b - 4.0 * a * c;
            if discriminant < 0.0 {
                return false;
            }

            ray_time = (-b + discriminant.sqrt()) / (2.0 * a);
            if ray_time < 0.0 {
                ray_time = (-b - discriminant.sqrt()) / (2.0 * a);
            }
        }

        if ray_time < 0.0 {
            return false;
        }

        let local_position = local_ray.calc_position(ray_time);
        if local_position.z < self.local_z_min || local_position.z > self.local_z_max {
            return false;
        }

        return ray_time < max_distance;
    }

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::IntersectableShapeSurface> {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let o = local_ray.origin();
        let d = local_ray.direction();
        let a = d.x * d.x + d.y * d.y;
        let b = 2.0 * (d.x * o.x + d.y * o.y);
        let c = o.x * o.x + o.y * o.y - self.local_radius * self.local_radius;

        let mut ray_time;
        if a == 0.0 {
            ray_time = -c / b;
        } else {
            let discriminant = b * b - 4.0 * a * c;
            if discriminant < 0.0 {
                return None;
            }

            ray_time = (-b + discriminant.sqrt()) / (2.0 * a);
            if ray_time < 0.0 {
                ray_time = (-b - discriminant.sqrt()) / (2.0 * a);
            }
        }

        if ray_time < 0.0 {
            return None;
        }

        let local_position = local_ray.calc_position(ray_time);
        if local_position.z < self.local_z_min || local_position.z > self.local_z_max {
            return None;
        }

        let local_normal = vec3::Vec3::new(local_position.x, local_position.y, 0.0);
        let local_dpdu = vec3::Vec3::new(
            -2.0 * math::PI_F32 * local_position.y,
            2.0 * math::PI_F32 * local_position.x,
            0.0,
        );
        let local_dpdv = vec3::Vec3::new(0.0, 0.0, self.local_z_max - self.local_z_min);

        return Some(shape::IntersectableShapeSurface::new(
            ray_time,
            local_position,
            local_normal,
            local_dpdu,
            local_dpdv,
            &self.object_to_world,
            &self.normal_transform,
        ));
    }
}

impl shape::SamplableShape for Cylinder {
    fn sample_surface(
        &self,
        _sample: &vec2::Vec2,
        _surface_point_ref: &vec3::Vec3,
        _surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        todo!()
    }
}
