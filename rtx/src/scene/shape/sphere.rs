use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: vec3::Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: vec3::Vec3, radius: f32) -> Sphere {
        return Sphere { center, radius };
    }

    pub fn center(&self) -> &vec3::Vec3 {
        return &self.center;
    }

    pub fn radius(&self) -> f32 {
        return self.radius;
    }
}

impl shape::Shape for Sphere {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        let center = self.center;
        let radius = self.radius;
        let radius_sq = radius * radius;

        let oc = center - *ray.origin();
        let oc_length_sq = vec3::Vec3::length_sq(&oc);
        let origin_outside = oc_length_sq >= radius_sq;

        let tca = vec3::Vec3::dot(&oc, ray.direction());
        if tca < 0.0 && origin_outside {
            return false;
        }

        let hc_length_sq = radius * radius - oc_length_sq + tca * tca;
        if hc_length_sq < 0.0 {
            return false;
        }

        let t: f32;
        if origin_outside {
            t = tca - f32::sqrt(hc_length_sq);
        } else {
            t = tca + f32::sqrt(hc_length_sq);
        }

        return t < max_distance;
    }

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::ShapeSurface> {
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

        let t: f32;
        if origin_outside {
            t = tca - f32::sqrt(hc_length_sq);
        } else {
            t = tca + f32::sqrt(hc_length_sq);
        }

        let position = ray.calc_position(t);
        let mut normal = position - self.center;
        normal = vec3::Vec3::normalize(&normal).unwrap();

        return Some(shape::ShapeSurface::new(t, position, normal));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;
    use crate::scene::shape::Shape;

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
    fn test_intersect_ray_not_intersect() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 1.0, 20.0),
            vec3::Vec3::new(0.0, 12.0, 0.0),
        );

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 4.0);
        assert!(vec3::Vec3::distance(ray.origin(), sphere.center()) > sphere.radius());

        let intersect = sphere.intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_intersect_ray_outside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 0.0, 10.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 2.0);
        assert!(vec3::Vec3::distance(ray.origin(), &sphere.center) > sphere.radius);

        match sphere.intersect_ray(&ray) {
            Some(shape_surface) => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(shape_surface.ray_time());
                assert!(math::equal_epsilon_f32(
                    shape_surface.position().x,
                    intersect_pos.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.position().y,
                    intersect_pos.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.position().z,
                    intersect_pos.z,
                    math::EPSILON_F32_5
                ));

                let distance = vec3::Vec3::distance(&sphere.center, &shape_surface.position());
                assert!(math::equal_epsilon_f32(
                    distance,
                    sphere.radius,
                    math::EPSILON_F32_5
                ));

                // make sure the normal points out
                let mut direction = *shape_surface.position() - sphere.center;
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::length(shape_surface.normal()),
                    1.0,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::dot(shape_surface.normal(), &direction),
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

        let sphere = Sphere::new(vec3::Vec3::from(1.0), 4.0);
        assert!(vec3::Vec3::distance(ray.origin(), &sphere.center) < sphere.radius);

        match sphere.intersect_ray(&ray) {
            Some(shape_surface) => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(shape_surface.ray_time());
                assert!(math::equal_epsilon_f32(
                    shape_surface.position().x,
                    intersect_pos.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.position().y,
                    intersect_pos.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.position().z,
                    intersect_pos.z,
                    math::EPSILON_F32_5
                ));

                let distance = vec3::Vec3::distance(&sphere.center, &shape_surface.position());
                assert!(math::equal_epsilon_f32(
                    distance,
                    sphere.radius,
                    math::EPSILON_F32_5
                ));

                // make sure the normal points outside
                let mut direction = *shape_surface.position() - sphere.center;
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::length(shape_surface.normal()),
                    1.0,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::dot(shape_surface.normal(), &direction),
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
