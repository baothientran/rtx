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

    pub fn uniform_sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        _surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        let outer_radius_sq = self.outer_radius * self.outer_radius;
        let inner_radius_sq = self.inner_radius * self.inner_radius;
        let r = f32::sqrt(sample.x * (outer_radius_sq - inner_radius_sq) + inner_radius_sq);
        let theta = 2.0 * math::PI_F32 * sample.y;
        let world_surface_point = (self.object_to_world
            * vec4::Vec4::new(r * f32::cos(theta), r * f32::sin(theta), 0.0, 1.0))
        .to_vec3();

        let maybe_world_normal = (self.normal_transform
            * vec4::Vec4::new(0.0, 0.0, 1.0, 0.0))
        .to_vec3()
        .normalize();
        if maybe_world_normal.is_none() {
            return None;
        }

        let world_normal = maybe_world_normal.unwrap();
        let direction = surface_point_ref - world_surface_point;
        let normalize_direction = direction.normalize().unwrap();
        let cos_theta = f32::max(world_normal.dot(&normalize_direction), 0.0);
        if cos_theta == 0.0 {
            return None;
        }

        let area = math::PI_F32
            * (self.outer_radius * self.outer_radius - self.inner_radius * self.inner_radius);

        let pdf = direction.length_sq() / (area * cos_theta);
        return Some(shape::SampleShapeSurface::new(pdf, world_surface_point));
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

impl shape::SamplableShape for Disk {
    fn sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        return self.uniform_sample_surface(sample, surface_point_ref, surface_normal_ref);
    }
}
