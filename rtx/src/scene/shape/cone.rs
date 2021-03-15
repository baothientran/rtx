use crate::core::mat4;
use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::ray;
use crate::scene::shape;
use std::mem;

pub struct Cone {
    object_to_world: mat4::Mat4,
    world_to_object: mat4::Mat4,
    normal_transform: mat4::Mat4,
    radius: f32,
    height: f32,
}

impl Cone {
    pub fn new(object_to_world: mat4::Mat4, radius: f32, height: f32) -> Cone {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();

        return Cone {
            object_to_world,
            world_to_object,
            normal_transform,
            radius,
            height,
        };
    }
}

impl shape::IntersectableShape for Cone {
    fn is_intersect(&self, ray: &crate::scene::ray::Ray, max_distance: f32) -> bool {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let o = local_ray.origin();
        let d = local_ray.direction();

        let height_sq = self.height * self.height;
        let radius_sq = self.radius * self.radius;
        let a = height_sq * (d.x * d.x + d.y * d.y) - radius_sq * d.z * d.z;
        let b =
            2.0 * height_sq * (o.x * d.x + o.y * d.y) - 2.0 * d.z * radius_sq * (o.z - self.height);
        let c = height_sq * (o.x * o.x + o.y * o.y)
            - radius_sq * (self.height - o.z) * (self.height - o.z);
        if a == 0.0 {
            return false;
        }

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }

        let mut ray_time_0 = (-b + discriminant.sqrt()) / (2.0 * a);
        let mut ray_time_1 = (-b - discriminant.sqrt()) / (2.0 * a);
        if ray_time_0 < ray_time_1 {
            if ray_time_0 < 0.0 {
                mem::swap(&mut ray_time_0, &mut ray_time_1);
            }
        } else if ray_time_1 > 0.0 {
            mem::swap(&mut ray_time_0, &mut ray_time_1);
        }

        let mut ray_time = ray_time_0;
        if ray_time < 0.0 {
            return false;
        }

        let mut local_position = local_ray.calc_position(ray_time);
        if local_position.z < 0.0 || local_position.z > self.height {
            ray_time = ray_time_1;
            if ray_time < 0.0 {
                return false;
            }

            local_position = local_ray.calc_position(ray_time);
            if local_position.z < 0.0 || local_position.z > self.height {
                return false;
            }
        }

        return ray_time < max_distance;
    }

    fn intersect_ray(
        &self,
        ray: &crate::scene::ray::Ray,
    ) -> Option<shape::IntersectableShapeSurface> {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let o = local_ray.origin();
        let d = local_ray.direction();

        let height_sq = self.height * self.height;
        let radius_sq = self.radius * self.radius;
        let a = height_sq * (d.x * d.x + d.y * d.y) - radius_sq * d.z * d.z;
        let b =
            2.0 * height_sq * (o.x * d.x + o.y * d.y) - 2.0 * d.z * radius_sq * (o.z - self.height);
        let c = height_sq * (o.x * o.x + o.y * o.y)
            - radius_sq * (self.height - o.z) * (self.height - o.z);
        if a == 0.0 {
            return None;
        }

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut ray_time_0 = (-b + discriminant.sqrt()) / (2.0 * a);
        let mut ray_time_1 = (-b - discriminant.sqrt()) / (2.0 * a);
        if ray_time_0 < ray_time_1 {
            if ray_time_0 < 0.0 {
                mem::swap(&mut ray_time_0, &mut ray_time_1);
            }
        } else if ray_time_1 > 0.0 {
            mem::swap(&mut ray_time_0, &mut ray_time_1);
        }

        let mut ray_time = ray_time_0;
        if ray_time < 0.0 {
            return None;
        }

        let mut local_position = local_ray.calc_position(ray_time);
        if local_position.z < 0.0 || local_position.z > self.height {
            ray_time = ray_time_1;
            if ray_time < 0.0 {
                return None;
            }

            local_position = local_ray.calc_position(ray_time);
            if local_position.z < 0.0 || local_position.z > self.height {
                return None;
            }
        }

        let v = local_position.z / self.height;
        let local_dpdu = vec3::Vec3::new(
            -2.0 * math::PI_F32 * local_position.y,
            2.0 * math::PI_F32 * local_position.x,
            0.0,
        );
        let local_dpdv = vec3::Vec3::new(
            -local_position.x / (1.0 - v),
            -local_position.y / (1.0 - v),
            self.height,
        );
        let mut local_normal = local_dpdu
            .cross(&local_dpdv)
            .normalize()
            .unwrap_or(vec3::Vec3::new(0.0, 0.0, 1.0));
        if local_ray.direction().dot(&local_normal) > 0.0 {
            local_normal = -local_normal;
        }

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

impl shape::SamplableShape for Cone {
    fn sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        _surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        let local_surface_point_ref =
            (self.world_to_object * vec4::Vec4::from_vec3(surface_point_ref, 1.0)).to_vec3();

        let z = self.height * (-f32::sqrt(sample.x));
        let radius = (self.radius / self.height) * z;
        let theta = 2.0 * math::PI_F32 * sample.y;

        let local_sample_point = vec3::Vec3::new(
            radius * f32::cos(theta),
            radius * f32::sin(theta),
            z + self.height,
        );

        // calculate local normal
        let v = local_sample_point.z / self.height;
        let local_dpdu = vec3::Vec3::new(
            -2.0 * math::PI_F32 * local_sample_point.y,
            2.0 * math::PI_F32 * local_sample_point.x,
            0.0,
        );
        let local_dpdv = vec3::Vec3::new(
            -local_sample_point.x / (1.0 - v),
            -local_sample_point.y / (1.0 - v),
            self.height,
        );
        let local_normal = local_dpdu.cross(&local_dpdv).normalize().unwrap();
        let direction = local_surface_point_ref - local_sample_point;
        let normalize_direction = direction.normalize().unwrap();
        let cos_theta = f32::abs(local_normal.dot(&normalize_direction));
        if cos_theta == 0.0 {
            return None;
        }

        let area = math::PI_F32
            * self.radius
            * (self.radius + f32::sqrt(self.height * self.height + self.radius * self.radius));

        let world_surface_point =
            (self.object_to_world * vec4::Vec4::from_vec3(&local_sample_point, 1.0)).to_vec3();
        let pdf = direction.length_sq() / (area * cos_theta);
        return Some(shape::SampleShapeSurface::new(pdf, world_surface_point));
    }
}
