use crate::core::mat4;
use crate::core::math;
use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

pub struct Plane {
    normal: vec3::Vec3,
    distance: f32,
    object_to_world: mat4::Mat4,
    world_to_object: mat4::Mat4,
    normal_transform: mat4::Mat4,
}

impl Plane {
    pub fn new(object_to_world: mat4::Mat4, normal: vec3::Vec3, distance: f32) -> Plane {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();
        return Plane {
            normal: vec3::Vec3::normalize(&normal).unwrap(),
            distance,
            object_to_world,
            world_to_object,
            normal_transform,
        };
    }
}

impl shape::Shape for Plane {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let vd = vec3::Vec3::dot(&self.normal, local_ray.direction());
        if math::equal_epsilon_f32(vd, 0.0, math::EPSILON_F32_6) {
            return false;
        }

        let vo = -vec3::Vec3::dot(&self.normal, local_ray.origin()) - self.distance;
        let t = vo / vd;
        return t > 0.0 && t < max_distance;
    }

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::ShapeSurface> {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let vd = vec3::Vec3::dot(&self.normal, local_ray.direction());
        if math::equal_epsilon_f32(vd, 0.0, math::EPSILON_F32_6) {
            return None;
        }

        let vo = -vec3::Vec3::dot(&self.normal, local_ray.origin()) - self.distance;
        let t = vo / vd;
        if t <= 0.0 {
            return None;
        }

        // calculate intersection point
        let local_position = local_ray.calc_position(t);

        // calculate dpdu and dpdv
        let mut local_dpdu = vec3::Vec3::from(0.0);
        let mut local_dpdv = vec3::Vec3::from(0.0);
        vec3::Vec3::coordinate_system(&self.normal, &mut local_dpdv, &mut local_dpdu);

        return Some(shape::ShapeSurface::new(
            t,
            local_position,
            self.normal,
            local_dpdu,
            local_dpdv,
            &self.object_to_world,
            &self.normal_transform,
        ));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scene::shape::Shape;
    use crate::core::vec4;

    #[test]
    fn test_intersect_ray_away() {
        let object_to_world = mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.0, 1.0, 1.0));
        let normal = vec3::Vec3::new(0.0, 1.0, 0.0); 
        let distance = 0.2;
        let plane = Plane::new(
            object_to_world,
            normal,
            distance,
        );

        let ray_origin = vec3::Vec3::new(0.0, -4.0, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), &plane.normal) > 0.0);

        match plane.intersect_ray(&ray) {
            None => {
                assert!(false);
            }
            Some(shape_surface) => {
                let transform_plane = mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap() * vec4::Vec4::from_vec3(&normal, distance); 
                let plane_func =
                    vec3::Vec3::dot(&shape_surface.calc_world_position(), &shape_surface.calc_world_normal())
                        + transform_plane.w;
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
        let object_to_world = mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.0, 1.0, 1.0));
        let normal = vec3::Vec3::new(0.0, 1.0, 0.0); 
        let distance = 0.2;
        let plane = Plane::new(
            object_to_world,
            normal,
            distance,
        );

        let ray_origin = vec3::Vec3::new(1.0, 1.0, 1.0);
        let mut ray_direction = vec3::Vec3::from(0.0) - ray_origin;
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), &plane.normal) < 0.0);

        match plane.intersect_ray(&ray) {
            None => {
                assert!(false);
            }
            Some(shape_surface) => {
                let transform_plane = mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap() * vec4::Vec4::from_vec3(&normal, distance); 
                let plane_func =
                    vec3::Vec3::dot(&shape_surface.calc_world_position(), &shape_surface.calc_world_normal())
                        + transform_plane.w;
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
        let plane = Plane::new(mat4::Mat4::new(), vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.2, 1.0);
        let ray_direction = vec3::Vec3::new(0.0, 0.0, -1.0);
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane.intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_intersect_ray_no_intersect() {
        let plane = Plane::new(mat4::Mat4::new(), vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.6, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane.intersect_ray(&ray);
        assert!(intersect.is_none());
    }
}
