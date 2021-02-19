use crate::core::mat4;
use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::ray;
use crate::scene::shape;
use crate::scene::shape::IntersectableShape;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    world_center: vec3::Vec3,
    world_radius: f32,
    radius: f32,
    object_to_world: mat4::Mat4,
    world_to_object: mat4::Mat4,
    normal_transform: mat4::Mat4,
    inverse_normal_transform: mat4::Mat4,
}

impl Sphere {
    pub fn new(object_to_world: mat4::Mat4, radius: f32) -> Sphere {
        let world_to_object = mat4::Mat4::inverse(&object_to_world).unwrap();
        let normal_transform =
            mat4::Mat4::inverse(&mat4::Mat4::transpose(&object_to_world)).unwrap();
        let inverse_normal_transform = normal_transform.inverse().unwrap();

        let world_center = (object_to_world * vec4::Vec4::new(0.0, 0.0, 0.0, 1.0)).to_vec3();
        let world_radius_vec3 = (object_to_world * vec4::Vec4::new(radius, 0.0, 0.0, 1.0)).to_vec3();
        let world_radius = (world_radius_vec3 - world_center).length();
        return Sphere {
            world_center,
            world_radius,
            radius,
            object_to_world,
            world_to_object,
            normal_transform,
            inverse_normal_transform
        };
    }

    fn behind_surface_tangent_plane(
        &self,
        surface_point: &vec3::Vec3,
        surface_normal: &vec3::Vec3,
    ) -> bool {
        let local_surface_point = (self.world_to_object * vec4::Vec4::from_vec3(surface_point, 1.0)).to_vec3();
        let local_surface_normal = (self.inverse_normal_transform * vec4::Vec4::from_vec3(surface_normal, 0.0)).to_vec3().normalize().unwrap();
        if (-local_surface_point).dot(&local_surface_normal) >= 0.0 {
            return false;
        }

        let distance_center_plane = -local_surface_point.dot(&local_surface_normal); 
        return f32::abs(distance_center_plane) >= self.radius;
    }
}

impl shape::IntersectableShape for Sphere {
    fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let radius = self.radius;
        let radius_sq = radius * radius;

        let oc = -local_ray.origin();
        let oc_length_sq = vec3::Vec3::length_sq(&oc);
        let origin_outside = oc_length_sq >= radius_sq;

        let tca = vec3::Vec3::dot(&oc, local_ray.direction());
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

    fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::IntersectableShapeSurface> {
        let local_ray = ray::Ray::transform(ray, &self.world_to_object);
        let radius = self.radius;
        let radius_sq = radius * radius;

        let oc = -local_ray.origin();
        let oc_length_sq = vec3::Vec3::length_sq(&oc);
        let origin_outside = oc_length_sq >= radius_sq;

        let tca = vec3::Vec3::dot(&oc, local_ray.direction());
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

        // calculate position and normal
        let local_position = local_ray.calc_position(t);
        let maybe_local_normal = vec3::Vec3::normalize(&local_position);
        if maybe_local_normal.is_none() {
            return None;
        }

        let local_normal = maybe_local_normal.unwrap();

        // calculate dpdu and dpdv
        let two_pi = math::PI_F32 * 2.0;
        let theta = f32::acos(math::clamp(local_position.y / self.radius, -1.0, 1.0));

        let inv_y_radius = 1.0
            / f32::sqrt(local_position.x * local_position.x + local_position.z * local_position.z);
        let sin_phi = local_position.z * inv_y_radius;
        let cos_phi = local_position.x * inv_y_radius;
        let local_dpdu =
            vec3::Vec3::new(-two_pi * local_position.z, 0.0, two_pi * local_position.x);
        let local_dpdv = math::PI_F32
            * vec3::Vec3::new(
                local_position.y * cos_phi,
                -self.radius * f32::sin(theta),
                local_position.y * sin_phi,
            );

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

impl shape::SamplableShape for Sphere {
    fn sample_surface(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        surface_normal_ref: &vec3::Vec3,
    ) -> Option<shape::SampleShapeSurface> {
        if self.behind_surface_tangent_plane(surface_point_ref, surface_normal_ref) {
            return None;
        }

        let distance_sq = (surface_point_ref - self.world_center).length_sq();
        let radius_sq = self.radius * self.radius;
        if distance_sq <= radius_sq {
            return None;
        }

        let cos_alpha =
            1.0 - sample.x + sample.x * f32::sqrt(f32::max(1.0 - radius_sq / distance_sq, 0.0));
        let sin_alpha = f32::sqrt(f32::max(1.0 - cos_alpha * cos_alpha, 0.0));

        let phi = 2.0 * math::PI_F32 * sample.y;
        let cos_phi = f32::cos(phi);
        let sin_phi = f32::sin(phi);

        let w = (self.world_center - surface_point_ref).normalize().unwrap();
        let v = w.cross(surface_normal_ref).normalize().unwrap();
        let u = v.cross(&w).normalize().unwrap();
        let sample_transform = mat4::Mat4::from_scalars(
            u.x, u.y, u.z, 0.0, v.x, v.y, v.z, 0.0, w.x, w.y, w.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let direction_vec4 = sample_transform
            * vec4::Vec4::new(cos_phi * sin_alpha, sin_phi * sin_alpha, cos_alpha, 0.0);

        let direction = direction_vec4.to_vec3().normalize().unwrap();
        let ray = ray::Ray::new(*surface_point_ref, direction);
        if let Some(shape_surface) = self.intersect_ray(&ray) {
            let n = 1.0 - f32::sqrt(f32::max(1.0 - radius_sq / distance_sq, 0.0));
            return Some(shape::SampleShapeSurface::new(
                1.0 / (2.0 * math::PI_F32 * n),
                shape_surface.calc_world_position(),
            ));
        }

        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;
    use crate::scene::shape::IntersectableShape;

    #[test]
    fn test_intersect_ray_not_intersect() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 1.0, 20.0),
            vec3::Vec3::new(0.0, 12.0, 0.0),
        );

        let center = vec3::Vec3::from(1.0);
        let radius = 4.0;
        let sphere = Sphere::new(mat4::Mat4::translate(&mat4::Mat4::new(), &center), radius);
        assert!(vec3::Vec3::distance(ray.origin(), &center) > radius);

        let intersect = sphere.intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_intersect_ray_outside() {
        let ray = ray::Ray::new(
            vec3::Vec3::new(0.0, 0.0, 10.0),
            vec3::Vec3::new(0.0, 0.0, -1.0),
        );

        let center = vec3::Vec3::from(1.0);
        let radius = 2.0;
        let sphere = Sphere::new(mat4::Mat4::translate(&mat4::Mat4::new(), &center), radius);
        assert!(vec3::Vec3::distance(ray.origin(), &center) > sphere.radius);

        match sphere.intersect_ray(&ray) {
            Some(shape_surface) => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(shape_surface.ray_time());
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_position().x,
                    intersect_pos.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_position().y,
                    intersect_pos.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_position().z,
                    intersect_pos.z,
                    math::EPSILON_F32_5
                ));

                let distance = vec3::Vec3::distance(&center, &shape_surface.calc_world_position());
                assert!(math::equal_epsilon_f32(
                    distance,
                    sphere.radius,
                    math::EPSILON_F32_5
                ));

                // make sure the normal points out
                let mut direction = shape_surface.calc_world_position() - center;
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::length(&shape_surface.calc_world_normal()),
                    1.0,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::dot(&shape_surface.calc_world_normal(), &direction),
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

        let center = vec3::Vec3::from(1.0);
        let radius = 4.0;
        let sphere = Sphere::new(mat4::Mat4::translate(&mat4::Mat4::new(), &center), radius);
        assert!(vec3::Vec3::distance(ray.origin(), &center) < sphere.radius);

        match sphere.intersect_ray(&ray) {
            Some(shape_surface) => {
                // verify the position is on the sphere
                let intersect_pos = ray.calc_position(shape_surface.ray_time());
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_position().x,
                    intersect_pos.x,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_position().y,
                    intersect_pos.y,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    shape_surface.calc_world_position().z,
                    intersect_pos.z,
                    math::EPSILON_F32_5
                ));

                let distance = vec3::Vec3::distance(&center, &shape_surface.calc_world_position());
                assert!(math::equal_epsilon_f32(
                    distance,
                    sphere.radius,
                    math::EPSILON_F32_5
                ));

                // make sure the normal points outside
                let mut direction = shape_surface.calc_world_position() - center;
                direction = vec3::Vec3::normalize(&direction).unwrap();
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::length(&shape_surface.calc_world_normal()),
                    1.0,
                    math::EPSILON_F32_5
                ));
                assert!(math::equal_epsilon_f32(
                    vec3::Vec3::dot(&shape_surface.calc_world_normal(), &direction),
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
