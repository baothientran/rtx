use crate::core::vec3;
use crate::scene::ray;
use crate::scene::renderable;
use crate::scene::shape::{plane, Shape};

pub struct RenderablePlane {
    plane: plane::Plane,
}

impl RenderablePlane {
    pub fn new(normal: vec3::Vec3, distance: f32) -> RenderablePlane {
        return RenderablePlane {
            plane: plane::Plane::new(normal, distance),
        };
    }
}

impl renderable::Renderable for RenderablePlane {
    fn intersect_ray(&self, ray: &ray::Ray) -> Option<renderable::SurfaceInfo> {
        match self.plane.intersect_ray(ray) {
            Some(t) => {
                return Some(renderable::SurfaceInfo {
                    ray_time: t,
                    position: ray.calc_position(t),
                    normal: *self.plane.normal(),
                });
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;
    use crate::scene::renderable::Renderable;

    #[test]
    fn test_create_plane() {
        let renderable_plane = RenderablePlane::new(vec3::Vec3::new(2.0, 0.0, 0.0), 1.0);
        let plane = renderable_plane.plane;
        let normal = plane.normal();
        let distance = plane.distance();
        assert!(math::equal_epsilon_f32(
            vec3::Vec3::length(normal),
            1.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(normal.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(normal.y, 0.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(normal.z, 0.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(distance, 1.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_intersect_ray_renderable_no_intersect() {
        let plane = RenderablePlane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.6, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let surface_info = plane.intersect_ray(&ray);
        assert!(surface_info.is_none());
    }

    #[test]
    fn test_intersect_ray_renderable_with_plane_point_away() {
        let plane = RenderablePlane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, -4.0, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), plane.plane.normal()) > 0.0);

        match plane.intersect_ray(&ray) {
            None => {
                assert!(false);
            }
            Some(surface_info) => {
                let plane_func = vec3::Vec3::dot(&surface_info.position, plane.plane.normal())
                    + plane.plane.distance();
                assert!(math::equal_epsilon_f32(
                    plane_func,
                    0.0,
                    math::EPSILON_F32_5
                ));

                assert!(math::equal_epsilon_f32(
                    surface_info.normal.x,
                    plane.plane.normal().x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.normal.y,
                    plane.plane.normal().y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.normal.z,
                    plane.plane.normal().z,
                    math::EPSILON_F32_5
                ));
            }
        }
    }

    #[test]
    fn test_intersect_ray_renderable_with_plane_point_toward() {
        let plane = RenderablePlane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(1.0, 1.0, 1.0);
        let mut ray_direction = vec3::Vec3::from(0.0) - ray_origin;
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), plane.plane.normal()) < 0.0);

        match plane.intersect_ray(&ray) {
            None => {
                assert!(false);
            }
            Some(surface_info) => {
                let plane_func = vec3::Vec3::dot(&surface_info.position, plane.plane.normal())
                    + plane.plane.distance();
                assert!(math::equal_epsilon_f32(
                    plane_func,
                    0.0,
                    math::EPSILON_F32_5
                ));

                assert!(math::equal_epsilon_f32(
                    surface_info.normal.x,
                    plane.plane.normal().x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.normal.y,
                    plane.plane.normal().y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.normal.z,
                    plane.plane.normal().z,
                    math::EPSILON_F32_5
                ));
            }
        }
    }
}
