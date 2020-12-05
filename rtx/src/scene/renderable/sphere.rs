use crate::core::vec3;
use crate::scene::ray;
use crate::scene::renderable;
use crate::scene::shape::{sphere, Shape};

pub struct RenderableSphere {
    sphere: sphere::Sphere,
}

impl RenderableSphere {
    pub fn new(center: vec3::Vec3, radius: f32) -> RenderableSphere {
        return RenderableSphere {
            sphere: sphere::Sphere::new(center, radius),
        };
    }
}

impl renderable::Renderable for RenderableSphere {
    fn intersect_ray(&self, ray: &ray::Ray) -> Option<renderable::SurfaceInfo> {
        match self.sphere.intersect_ray(ray) {
            Some(t) => {
                let position = ray.calc_position(t);
                let mut normal = position - *self.sphere.center();
                normal = vec3::Vec3::normalize(&normal).unwrap();

                return Some(renderable::SurfaceInfo {
                    ray_time: t,
                    position,
                    normal,
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
    fn test_renderable_intersect_ray_outside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 0.0, 10.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let sphere = RenderableSphere::new(vec3::Vec3::from(1.0), 2.0);
        assert!(
            vec3::Vec3::distance(ray.origin(), sphere.sphere.center()) > sphere.sphere.radius()
        );

        match sphere.intersect_ray(&ray) {
            Some(surface_info) => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(surface_info.ray_time);
                assert!(math::equal_epsilon_f32(
                    surface_info.position.x,
                    intersect_pos.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.position.y,
                    intersect_pos.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.position.z,
                    intersect_pos.z,
                    math::EPSILON_F32_5
                ));

                let distance = vec3::Vec3::distance(sphere.sphere.center(), &surface_info.position);
                assert!(math::equal_epsilon_f32(
                    distance,
                    sphere.sphere.radius(),
                    math::EPSILON_F32_5
                ));

                // make sure the normal points out
                let mut direction = surface_info.position - *sphere.sphere.center();
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::length(&surface_info.normal),
                    1.0,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::dot(&surface_info.normal, &direction),
                    1.0,
                    math::EPSILON_F32_5
                ));
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_renderable_intersect_ray_inside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 1.0, 1.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let sphere = RenderableSphere::new(vec3::Vec3::from(1.0), 4.0);
        assert!(
            vec3::Vec3::distance(ray.origin(), sphere.sphere.center()) < sphere.sphere.radius()
        );

        match sphere.intersect_ray(&ray) {
            Some(surface_info) => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(surface_info.ray_time);
                assert!(math::equal_epsilon_f32(
                    surface_info.position.x,
                    intersect_pos.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.position.y,
                    intersect_pos.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    surface_info.position.z,
                    intersect_pos.z,
                    math::EPSILON_F32_5
                ));

                let distance = vec3::Vec3::distance(sphere.sphere.center(), &surface_info.position);
                assert!(math::equal_epsilon_f32(
                    distance,
                    sphere.sphere.radius(),
                    math::EPSILON_F32_5
                ));

                // make sure the normal points outside
                let mut direction = surface_info.position - *sphere.sphere.center();
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::length(&surface_info.normal),
                    1.0,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::dot(&surface_info.normal, &direction),
                    1.0,
                    math::EPSILON_F32_5
                ));
            }
            _ => {
                assert!(false);
            }
        }
    }
}
