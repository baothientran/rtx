use crate::core::mat4;
use crate::core::math;
use crate::core::vec3;
use crate::scene::ray;
use crate::scene::shape;

pub struct Plane {
    pub normal: vec3::Vec3,
    pub distance: f32,
    pub object_to_world: mat4::Mat4,
    pub world_to_object: mat4::Mat4,
    pub normal_transform: mat4::Mat4,
}

impl Plane {
    pub fn new(object_to_world: mat4::Mat4, normal: vec3::Vec3, distance: f32) -> Plane {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();

        return Plane {
            normal,
            distance,
            object_to_world,
            world_to_object,
            normal_transform,
        };
    }
}

impl shape::IntersectableShape for Plane {
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

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::IntersectableShapeSurface> {
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
        let mut local_normal = self.normal;
        if local_ray.direction().dot(&local_normal) > 0.0 {
            local_normal = -local_normal;
        }
        let mut local_dpdu = vec3::Vec3::from(0.0);
        let mut local_dpdv = vec3::Vec3::from(0.0);
        vec3::Vec3::coordinate_system(&local_normal, &mut local_dpdv, &mut local_dpdu);

        return Some(shape::IntersectableShapeSurface::new(
            t,
            local_position,
            local_normal,
            local_dpdu,
            local_dpdv,
            &self.object_to_world,
            &self.normal_transform,
        ));
    }
}
