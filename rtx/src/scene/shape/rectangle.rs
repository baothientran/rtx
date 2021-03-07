use crate::core::mat4;
use crate::core::vec2;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::ray;
use crate::scene::shape;

pub struct Rectangle {
    object_to_world: mat4::Mat4,
    world_to_object: mat4::Mat4,
    normal_transform: mat4::Mat4,
    inverse_normal_transform: mat4::Mat4,
    normal: vec3::Vec3,
    width: f32,
    height: f32,
    local_north_west_corner: vec3::Vec3,
    local_south_west_corner: vec3::Vec3,
    local_north_east_corner: vec3::Vec3,
    local_south_east_corner: vec3::Vec3,
}

impl Rectangle {
    pub fn new(object_to_world: mat4::Mat4, width: f32, height: f32) -> Rectangle {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();
        let inverse_normal_transform = normal_transform.inverse().unwrap();
        let normal = vec3::Vec3::new(0.0, 0.0, 1.0);
        let half_width = width * 0.5;
        let half_height = height * 0.5;
        let local_north_west_corner = vec3::Vec3::new(-half_width, half_height, 0.0);
        let local_south_west_corner = vec3::Vec3::new(-half_width, -half_height, 0.0);
        let local_north_east_corner = vec3::Vec3::new(half_width, half_height, 0.0);
        let local_south_east_corner = vec3::Vec3::new(half_width, -half_height, 0.0);

        return Rectangle {
            object_to_world,
            world_to_object,
            normal_transform,
            inverse_normal_transform,
            normal,
            width,
            height,
            local_north_west_corner,
            local_south_west_corner,
            local_north_east_corner,
            local_south_east_corner,
        };
    }

    fn completely_behind_surface_tangent_plane(
        &self,
        local_surface_point: &vec3::Vec3,
        local_surface_normal: &vec3::Vec3,
    ) -> bool {
        return (self.local_north_west_corner - local_surface_point).dot(local_surface_normal)
            < 0.0
            && (self.local_south_west_corner - local_surface_point).dot(local_surface_normal)
                < 0.0
            && (self.local_north_east_corner - local_surface_point).dot(local_surface_normal)
                < 0.0
            && (self.local_south_east_corner - local_surface_point).dot(local_surface_normal)
                < 0.0;
    }
}

impl shape::IntersectableShape for Rectangle {
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
        if local_position.x < -self.width * 0.5 || local_position.x > self.width * 0.5 {
            return false;
        }

        if local_position.y < -self.height * 0.5 || local_position.y > self.height * 0.5 {
            return false;
        }

        return ray_time < max_distance;
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
        if local_position.x < -self.width * 0.5 || local_position.x > self.width * 0.5 {
            return None;
        }

        if local_position.y < -self.height * 0.5 || local_position.y > self.height * 0.5 {
            return None;
        }

        let mut local_normal = self.normal;
        if local_ray.direction().dot(&local_normal) > 0.0 {
            local_normal = -local_normal;
        }

        let mut local_dpdu = vec3::Vec3::from(0.0);
        let mut local_dpdv = vec3::Vec3::from(0.0);
        vec3::Vec3::coordinate_system(&local_normal, &mut local_dpdv, &mut local_dpdu);

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

impl shape::SamplableShape for Rectangle {
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

        let local_sample_point =
            vec3::Vec3::new(sample.x * self.width, sample.y * self.height, 0.0)
                - vec3::Vec3::new(self.width * 0.5, self.height * 0.5, 0.0);
        let direction = local_surface_point_ref - local_sample_point;
        let normalize_direction = direction.normalize().unwrap();

        let cos_theta = f32::max(self.normal.dot(&normalize_direction), 0.0);
        if cos_theta == 0.0 {
            return None;
        }

        let world_surface_point = (self.object_to_world
            * vec4::Vec4::from_vec3(&local_sample_point, 1.0))
        .to_vec3();
        let pdf = direction.length_sq() / (self.width * self.height * cos_theta);
        return Some(shape::SampleShapeSurface::new(pdf, world_surface_point));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;
    use crate::core::vec4;
    use crate::scene::shape::IntersectableShape;

    #[test]
    fn test_intersect_ray_away() {
        let object_to_world =
            mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.0, 1.0, 1.0));
        let normal = vec3::Vec3::new(0.0, 1.0, 0.0);
        let distance = 0.2;
        let rectangle = Rectangle::new(
            mat4::Mat4::new()
                .translate(&vec3::Vec3::new(0.0, 1.0, 1.0))
                .translate(&vec3::Vec3::new(0.0, -0.2, 0.0))
                .rotate(
                    math::degree_to_radian(-90.0),
                    &vec3::Vec3::new(1.0, 0.0, 0.0),
                ),
            100.0,
            100.0,
        );

        let ray_origin = vec3::Vec3::new(0.0, -4.0, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), &rectangle.normal) > 0.0);

        match rectangle.intersect_ray(&ray) {
            None => {
                assert!(false);
            }
            Some(shape_surface) => {
                let transform_plane = mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world))
                    .unwrap()
                    * vec4::Vec4::from_vec3(&normal, distance);

                // normal is flipped
                let plane_func = vec3::Vec3::dot(
                    &shape_surface.calc_world_position(),
                    &shape_surface.calc_world_normal(),
                ) - transform_plane.w;
                assert!(math::equal_epsilon_f32(
                    plane_func,
                    0.0,
                    math::EPSILON_F32_5
                ));

                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_normal().x,
                    transform_plane.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_normal().y,
                    -transform_plane.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_normal().z,
                    transform_plane.z,
                    math::EPSILON_F32_5
                ));
            }
        }
    }

    #[test]
    fn test_intersect_ray_toward() {
        let object_to_world =
            mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.0, 1.0, 1.0));
        let normal = vec3::Vec3::new(0.0, 1.0, 0.0);
        let distance = 0.2;
        let rectangle = Rectangle::new(
            mat4::Mat4::new()
                .translate(&vec3::Vec3::new(0.0, 1.0, 1.0))
                .translate(&vec3::Vec3::new(0.0, -0.2, 0.0))
                .rotate(
                    math::degree_to_radian(-90.0),
                    &vec3::Vec3::new(1.0, 0.0, 0.0),
                ),
            100.0,
            100.0,
        );

        let ray_origin = vec3::Vec3::new(1.0, 1.0, 1.0);
        let mut ray_direction = vec3::Vec3::from(0.0) - ray_origin;
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), &rectangle.normal) < 0.0);

        match rectangle.intersect_ray(&ray) {
            None => {
                assert!(false);
            }
            Some(shape_surface) => {
                let transform_plane = mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world))
                    .unwrap()
                    * vec4::Vec4::from_vec3(&normal, distance);
                let plane_func = vec3::Vec3::dot(
                    &shape_surface.calc_world_position(),
                    &shape_surface.calc_world_normal(),
                ) + transform_plane.w;
                assert!(math::equal_epsilon_f32(
                    plane_func,
                    0.0,
                    math::EPSILON_F32_5
                ));

                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_normal().x,
                    transform_plane.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_normal().y,
                    transform_plane.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_normal().z,
                    transform_plane.z,
                    math::EPSILON_F32_5
                ));
            }
        }
    }

    #[test]
    fn test_intersect_ray_parallel() {
        let plane = Rectangle::new(
            mat4::Mat4::new().rotate(
                math::degree_to_radian(-90.0),
                &vec3::Vec3::new(1.0, 0.0, 0.0),
            ),
            1.0,
            1.0,
        );

        let ray_origin = vec3::Vec3::new(0.0, 0.0, 1.0);
        let ray_direction = vec3::Vec3::new(0.0, 0.0, -1.0);
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane.intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_intersect_ray_no_intersect() {
        let plane = Rectangle::new(
            mat4::Mat4::new()
                .translate(&vec3::Vec3::new(0.0, -0.2, 0.0))
                .rotate(
                    math::degree_to_radian(-90.0),
                    &vec3::Vec3::new(1.0, 0.0, 0.0),
                ),
            1.0,
            1.0,
        );

        let ray_origin = vec3::Vec3::new(0.0, 0.6, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane.intersect_ray(&ray);
        assert!(intersect.is_none());
    }
}
