use crate::core::vec3;
use crate::scene::ray;
use crate::scene::renderable;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: vec3::Vec3,
    radius: f32,
}

#[derive(Copy, Clone, Debug)]
struct SphereIntersect {
    pub t: f32,
    pub outside: bool
}

impl Sphere {
    pub fn new(center: vec3::Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn center(&self) -> &vec3::Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    fn _calc_normal(&self, p: &vec3::Vec3) -> vec3::Vec3 {
        let n = *p - self.center;
        return vec3::Vec3::normalize(&n).unwrap();
    }

    fn _intersect_ray(&self, ray: &ray::Ray) -> Option<SphereIntersect> {
        let center = self.center;
        let radius = self.radius;
        let radius_sq = radius * radius;

        let oc = center - *ray.origin();
        let oc_length_sq = vec3::Vec3::length_sq(&oc);
        let origin_outside = oc_length_sq >= radius_sq;

        let tca = vec3::Vec3::dot(&oc, ray.direction());
        if tca < 0.0 && origin_outside {
            return None;
        }

        let hc_length_sq = radius * radius - oc_length_sq + tca * tca;
        if hc_length_sq < 0.0 {
            return None;
        }

        if origin_outside {
            let t = tca - f32::sqrt(hc_length_sq);
            return Some(SphereIntersect{t, outside: origin_outside});
        }

        let t = tca + f32::sqrt(hc_length_sq);
        return Some(SphereIntersect{t, outside: origin_outside});
    }
}

impl renderable::Renderable for Sphere {
    fn intersect_ray(&self, ray: &ray::Ray, surface_info: &mut renderable::SurfaceInfo) {
        match self._intersect_ray(ray) {
            Some(intersect) => match surface_info {
                renderable::SurfaceInfo::None => {
                    let position = ray.calc_position(intersect.t);
                    let mut normal = self._calc_normal(&position);
                    if !intersect.outside {
                        normal = -normal;
                    }

                    *surface_info = renderable::SurfaceInfo::Hit {
                        ray_time: intersect.t,
                        position,
                        normal,
                    };
                }
                renderable::SurfaceInfo::Hit {
                    ray_time,
                    position,
                    normal,
                } => {
                    if *ray_time > intersect.t {
                        *ray_time = intersect.t;
                        *position = ray.calc_position(intersect.t);
                        *normal = self._calc_normal(position);
                        if !intersect.outside {
                            *normal = -*normal;
                    }
                    }
                }
            },
            None => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;
    use crate::scene::renderable::{Renderable, SurfaceInfo};

    #[test]
    fn test_create_sphere() {
        let sphere = Sphere::new(vec3::Vec3::new(1.0, 2.0, 3.0), 32.0);
        let center = sphere.center();
        let radius = sphere.radius();
        assert!(math::equal_epsilon_f32(center.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.y, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.z, 3.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(radius, 32.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_intersect_ray_outside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 0.0, 10.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 2.0);
        assert!(vec3::Vec3::distance(ray.origin(), sphere.center()) > sphere.radius());

        let intersect = sphere._intersect_ray(&ray).unwrap();
        let intersect_pos = ray.calc_position(intersect.t);
        let distance = vec3::Vec3::distance(sphere.center(), &intersect_pos);
        assert!(math::equal_epsilon_f32(distance, sphere.radius(), math::EPSILON_F32_5));
        assert!(intersect.outside);
    }

    #[test]
    fn test_intersect_ray_inside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 1.0, 1.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 4.0);
        assert!(vec3::Vec3::distance(ray.origin(), sphere.center()) < sphere.radius());

        let intersect = sphere._intersect_ray(&ray).unwrap();
        let intersect_pos = ray.calc_position(intersect.t);
        let distance = vec3::Vec3::distance(sphere.center(), &intersect_pos);
        assert!(math::equal_epsilon_f32(distance, sphere.radius(), math::EPSILON_F32_5));
        assert!(!intersect.outside);
    }

    #[test]
    fn test_intersect_ray_not_intersect() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 1.0, 20.0),
            vec3::Vec3::new(0.0, 12.0, 0.0),
        );

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 4.0);
        assert!(vec3::Vec3::distance(ray.origin(), sphere.center()) > sphere.radius());

        let intersect = sphere._intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_calc_normal() {
        let sphere = Sphere::new(vec3::Vec3::from(1.0), 4.0);
        let position = vec3::Vec3::from(2.0);
        let normal = sphere._calc_normal(&position);
        let mut direction = position - *sphere.center();
        direction = vec3::Vec3::normalize(&direction).unwrap();
        assert!(math::equal_epsilon_f32(vec3::Vec3::length(&normal), 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(vec3::Vec3::dot(&normal, &direction), 1.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_renderable_intersect_ray_outside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 0.0, 10.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 2.0);
        assert!(vec3::Vec3::distance(ray.origin(), sphere.center()) > sphere.radius());

        let mut surface_info = SurfaceInfo::None;
        sphere.intersect_ray(&ray, &mut surface_info);
        assert!(!surface_info.is_none());
        match surface_info {
            SurfaceInfo::Hit{ray_time, position, normal} => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(ray_time);
                assert!(math::equal_epsilon_f32(position.x, intersect_pos.x, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(position.y, intersect_pos.y, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(position.z, intersect_pos.z, math::EPSILON_F32_5));

                let distance = vec3::Vec3::distance(sphere.center(), &position);
                assert!(math::equal_epsilon_f32(distance, sphere.radius(), math::EPSILON_F32_5));

                // make sure the normal points out
                let mut direction = position - *sphere.center();
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(vec3::Vec3::length(&normal), 1.0, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(vec3::Vec3::dot(&normal, &direction), 1.0, math::EPSILON_F32_5));
            },
            _ => {}
        }
    }

    #[test]
    fn test_renderable_intersect_ray_inside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 1.0, 1.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 4.0);
        assert!(vec3::Vec3::distance(ray.origin(), sphere.center()) < sphere.radius());

        let mut surface_info = SurfaceInfo::None;
        sphere.intersect_ray(&ray, &mut surface_info);
        assert!(!surface_info.is_none());
        match surface_info {
            SurfaceInfo::Hit{ray_time, position, normal} => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(ray_time);
                assert!(math::equal_epsilon_f32(position.x, intersect_pos.x, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(position.y, intersect_pos.y, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(position.z, intersect_pos.z, math::EPSILON_F32_5));

                let distance = vec3::Vec3::distance(sphere.center(), &position);
                assert!(math::equal_epsilon_f32(distance, sphere.radius(), math::EPSILON_F32_5));

                // make sure the normal points inside
                let mut direction = position - *sphere.center();
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(vec3::Vec3::length(&normal), 1.0, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(vec3::Vec3::dot(&normal, &direction), -1.0, math::EPSILON_F32_5));
            },
            _ => {}
        }
    }
}
