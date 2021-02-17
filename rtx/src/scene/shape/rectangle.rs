use crate::core::mat4;
use crate::core::vec2;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::ray;
use crate::scene::shape;
use crate::scene::shape::plane;

pub struct Rectangle {
    plane: plane::Plane,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(object_to_world: mat4::Mat4, width: f32, height: f32) -> Rectangle {
        let normal = vec3::Vec3::new(0.0, 0.0, 1.0);
        let distance = 0.0;
        let plane = plane::Plane::new(object_to_world, normal, distance);

        return Rectangle {
            plane,
            width,
            height,
        };
    }

    pub fn completely_behind_surface_tangent_plane(&self, surface_point: &vec3::Vec3, surface_normal: &vec3::Vec3) -> bool {
        todo!()
    }
}

impl shape::IntersectableShape for Rectangle {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        let maybe_intersect = self.plane.intersect_ray(ray);
        if maybe_intersect.is_none() {
            return false;
        }

        let intersect = maybe_intersect.unwrap();

        // calculate intersection point
        let local_position = intersect.position;
        if local_position.x < -self.width * 0.5 || local_position.x > self.width * 0.5 {
            return false;
        }

        if local_position.y < -self.height * 0.5 || local_position.y > self.height * 0.5 {
            return false;
        }

        return intersect.ray_time < max_distance;
    }

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::IntersectableShapeSurface> {
        let maybe_intersect = self.plane.intersect_ray(ray);
        if maybe_intersect.is_none() {
            return None;
        }

        let intersect = maybe_intersect.unwrap();
        let local_position = intersect.position;
        if local_position.x < -self.width * 0.5 || local_position.x > self.width * 0.5 {
            return None;
        }

        if local_position.y < -self.height * 0.5 || local_position.y > self.height * 0.5 {
            return None;
        }

        return Some(intersect);
    }
}

impl shape::SamplableShape for Rectangle {
    fn sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        if self.completely_behind_surface_tangent_plane(surface_point_ref, surface_normal_ref) {
            return None;
        }

        let maybe_world_normal = (self.plane.normal_transform
            * vec4::Vec4::from_vec3(&self.plane.normal, 0.0))
        .to_vec3()
        .normalize();
        if maybe_world_normal.is_none() {
            return None;
        }

        let world_normal = maybe_world_normal.unwrap();
        let local_point = vec3::Vec3::new(sample.x * self.width, sample.y * self.height, 0.0)
            - vec3::Vec3::new(self.width * 0.5, self.height * 0.5, 0.0);
        let world_surface_point =
            (self.plane.object_to_world * vec4::Vec4::from_vec3(&local_point, 1.0)).to_vec3();
        let direction = surface_point_ref - world_surface_point;
        let normalize_direction = direction.normalize().unwrap();

        let cos_theta = f32::max(world_normal.dot(&normalize_direction), 0.0);
        if cos_theta == 0.0 {
            return None;
        }

        return Some(shape::SampleShapeSurface::new(
            direction.length_sq() / (self.width * self.height * cos_theta),
            world_surface_point,
        ));
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

        assert!(vec3::Vec3::dot(ray.direction(), &rectangle.plane.normal) > 0.0);

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

        assert!(vec3::Vec3::dot(ray.direction(), &rectangle.plane.normal) < 0.0);

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
