use crate::core::math;
use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

pub struct Plane {
    normal: vec3::Vec3,
    distance: f32,
}

impl Plane {
    pub fn new(normal: vec3::Vec3, distance: f32) -> Plane {
        Plane {
            normal: vec3::Vec3::normalize(&normal).unwrap(),
            distance,
        }
    }

    pub fn normal(&self) -> &vec3::Vec3 {
        return &self.normal;
    }

    pub fn distance(&self) -> f32 {
        return self.distance;
    }
}

impl shape::Shape for Plane {
    fn intersect_ray(&self, ray: &ray::Ray) -> Option<f32> {
        let vd = vec3::Vec3::dot(&self.normal, ray.direction());
        if math::equal_epsilon_f32(vd, 0.0, math::EPSILON_F32_6) {
            return None;
        }

        let vo = -vec3::Vec3::dot(&self.normal, ray.origin()) - self.distance;
        let t = vo / vd;
        if t <= 0.0 {
            return None;
        }

        return Some(t);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scene::shape::Shape;

    #[test]
    fn test_create_plane() {
        let plane = Plane::new(vec3::Vec3::new(2.0, 0.0, 0.0), 1.0);
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
    fn test_intersect_ray() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 1.0, 1.0);
        let mut ray_direction = vec3::Vec3::from(0.0) - ray_origin;
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane.intersect_ray(&ray).unwrap();
        let position = ray.calc_position(intersect);

        let plane_func = vec3::Vec3::dot(&position, plane.normal()) + plane.distance();
        assert!(math::equal_epsilon_f32(
            plane_func,
            0.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_intersect_ray_parallel() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.2, 1.0);
        let ray_direction = vec3::Vec3::new(0.0, 0.0, -1.0);
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane.intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_intersect_ray_no_intersect() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.6, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane.intersect_ray(&ray);
        assert!(intersect.is_none());
    }
}
