use crate::core::intersect;
use crate::core::ray;
use crate::core::sphere;
use crate::core::vec3;

pub struct HitRecord {
    pub ray_t: f32,
    pub position: vec3::Vec3,
    pub normal: vec3::Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray) -> Option<HitRecord>;
}

impl Hittable for sphere::Sphere {
    fn hit(&self, ray: &ray::Ray) -> Option<HitRecord> {
        match intersect::intersect_ray_sphere(&ray, &self) {
            Some(t) => {
                let position = ray.calc_position(t);
                let normal = self.calc_normal(&position);

                return Some(HitRecord {
                    ray_t: t,
                    normal,
                    position,
                });
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
