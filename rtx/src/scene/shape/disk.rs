use crate::core::mat4;
use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::ray;
use crate::scene::shape;

pub struct Disk {
    inner_radius: f32,
    outer_radius: f32,
    object_to_world: mat4::Mat4,
    world_to_object: mat4::Mat4,
    normal_transform: mat4::Mat4,
    inverse_normal_transform: mat4::Mat4,
}

impl Disk {
    pub fn new(object_to_world: mat4::Mat4, inner_radius: f32, outer_radius: f32) -> Disk {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();
        let inverse_normal_transform = normal_transform.inverse().unwrap();

        return Disk {
            inner_radius,
            outer_radius,
            object_to_world,
            world_to_object,
            normal_transform,
            inverse_normal_transform,
        };
    }

    pub fn uniform_sample_local_surface(&self, sample: &vec2::Vec2) -> Option<vec3::Vec3> {
        let outer_radius_sq = self.outer_radius * self.outer_radius;
        let inner_radius_sq = self.inner_radius * self.inner_radius;
        let r = f32::sqrt(sample.x * (outer_radius_sq - inner_radius_sq) + inner_radius_sq);
        let theta = 2.0 * math::PI_F32 * sample.y;
        let local_sample_point = vec3::Vec3::new(r * f32::cos(theta), r * f32::sin(theta), 0.0);

        return Some(local_sample_point);
    }

    pub fn concentric_sample_local_surface(&self, sample: &vec2::Vec2) -> Option<vec3::Vec3> {
        let mut r;
        let theta;
        let offset = 2.0 * sample - vec2::Vec2::from(1.0);
        if offset.x == 0.0 || offset.y == 0.0 {
            r = 0.0;
            theta = 0.0;
        } else if offset.x * offset.x > offset.y * offset.y {
            r = offset.x;
            theta = math::PI_F32 * 0.25 * offset.y / offset.x;
        } else {
            r = offset.y;
            theta = math::PI_F32 * 0.5 - math::PI_F32 * 0.25 * offset.x / offset.y;
        }

        r *= self.outer_radius;
        if r.abs() < self.inner_radius {
            return None;
        }

        let local_sample_point = vec3::Vec3::new(r * f32::cos(theta), r * f32::sin(theta), 0.0);
        return Some(local_sample_point);
    }

    fn completely_behind_surface_tangent_plane(
        &self,
        local_surface_point: &vec3::Vec3,
        local_surface_normal: &vec3::Vec3,
    ) -> bool {
        if (-local_surface_point).dot(&local_surface_normal) >= 0.0 {
            return false;
        }

        // tangent plane is parallel with disk
        if local_surface_normal.x == 0.0 && local_surface_normal.y == 0.0 {
            return local_surface_point.z >= 0.0;
        }

        let tangent_plane_constant = local_surface_point.dot(&local_surface_normal);
        let mut p1 = vec3::Vec3::new(0.0, 0.0, 0.0);
        if local_surface_normal.y != 0.0 {
            p1.y = tangent_plane_constant / local_surface_normal.y;
        }

        let mut p2 = vec3::Vec3::new(0.0, 0.0, 0.0);
        if local_surface_normal.x != 0.0 {
            p2.x = tangent_plane_constant / local_surface_normal.x;
        }

        let p1p2 = p2 - p1;
        let distance = (-p1).cross(&p1p2).length() / p1p2.length();

        return distance >= self.outer_radius;
    }
}

impl shape::IntersectableShape for Disk {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
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

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::IntersectableShapeSurface> {
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

impl shape::SamplableShape for Disk {
    fn sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        let local_surface_point_ref =
            (self.world_to_object * vec4::Vec4::from_vec3(surface_point_ref, 1.0)).to_vec3();
        let local_surface_normal_ref = (self.inverse_normal_transform
            * vec4::Vec4::from_vec3(surface_normal_ref, 0.0))
        .to_vec3()
        .normalize()
        .unwrap();
        if self.completely_behind_surface_tangent_plane(
            &local_surface_point_ref,
            &local_surface_normal_ref,
        ) {
            return None;
        }

        if let Some(local_sample_point) = self.uniform_sample_local_surface(sample) {
            let local_normal = vec3::Vec3::new(0.0, 0.0, 1.0);
            let direction = local_surface_point_ref - local_sample_point;
            let normalize_direction = direction.normalize().unwrap();
            let cos_theta = f32::max(local_normal.dot(&normalize_direction), 0.0);
            if cos_theta == 0.0 {
                return None;
            }

            let area = math::PI_F32
                * (self.outer_radius * self.outer_radius - self.inner_radius * self.inner_radius);

            let world_surface_point =
                (self.object_to_world * vec4::Vec4::from_vec3(&local_sample_point, 1.0)).to_vec3();
            let pdf = direction.length_sq() / (area * cos_theta);
            return Some(shape::SampleShapeSurface::new(pdf, world_surface_point));
        }

        return None;
    }
}
