use crate::core::ray;
use crate::core::sphere;
use crate::core::vec3;

pub fn intersect_ray_sphere(ray: &ray::Ray, sphere: &sphere::Sphere) -> Option<f32> {
    let center = sphere.center();
    let radius = sphere.radius();
    let radius_sq = radius * radius;

    let oc = *center - *ray.origin();
    let oc_length_sq = vec3::Vec3::length_sq(&oc);
    let origin_outside = oc_length_sq > radius_sq;

    let tca = vec3::Vec3::dot(&oc, ray.direction());
    if tca < 0.0 && origin_outside {
        return None;
    }

    let hc_length_sq = radius * radius - oc_length_sq + tca * tca;
    if origin_outside {
        return Some(tca - f32::sqrt(hc_length_sq));
    }

    return Some(tca + f32::sqrt(hc_length_sq));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect_ray_sphere() {}
}
