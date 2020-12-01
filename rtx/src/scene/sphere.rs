use crate::core::vec3;
use crate::scene::ray;
use crate::scene::renderable;

pub struct Sphere {
    center: vec3::Vec3,
    radius: f32,
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

    pub fn calc_normal(&self, p: &vec3::Vec3) -> vec3::Vec3 {
        let n = *p - self.center;
        return vec3::Vec3::normalize(&n).unwrap();
    }

    pub fn intersect_ray(&self, ray: &ray::Ray) -> Option<f32> {
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
            return Some(tca - f32::sqrt(hc_length_sq));
        }

        return Some(tca + f32::sqrt(hc_length_sq));
    }
}

impl renderable::Renderable for Sphere {
    fn intersect_ray(&self, ray: &ray::Ray, surface_info: &mut renderable::SurfaceInfo) {
        match self.intersect_ray(ray) {
            Some(t) => match surface_info {
                renderable::SurfaceInfo::None => {
                    let position = ray.calc_position(t);
                    let normal = self.calc_normal(&position);
                    *surface_info = renderable::SurfaceInfo::Hit {
                        ray_time: t,
                        position,
                        normal,
                    };
                }
                renderable::SurfaceInfo::Hit {
                    ray_time,
                    position,
                    normal,
                } => {
                    if *ray_time > t {
                        *ray_time = t;
                        *position = ray.calc_position(t);
                        *normal = self.calc_normal(position);
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
    fn test_intersect_ray() {
    }

    #[test]
    fn test_calc_normal() {}

    #[test]
    fn test_renderable_intersect_ray() {}
}
