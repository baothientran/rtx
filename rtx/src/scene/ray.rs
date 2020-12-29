use crate::core::mat4;
use crate::core::vec3;
use crate::core::vec4;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
        let norm_direction = vec3::Vec3::normalize(&direction).unwrap();
        return Ray {
            origin,
            direction: norm_direction,
        };
    }

    pub fn origin(&self) -> &vec3::Vec3 {
        return &self.origin;
    }

    pub fn direction(&self) -> &vec3::Vec3 {
        return &self.direction;
    }

    pub fn calc_position(&self, t: f32) -> vec3::Vec3 {
        return self.origin + t * self.direction;
    }

    pub fn transform(ray: &Ray, mat: &mat4::Mat4) -> Ray {
        return Ray::new(
            vec4::Vec4::to_vec3(&(mat * &vec4::Vec4::from_vec3(&ray.origin, 1.0))),
            vec4::Vec4::to_vec3(&(mat * &vec4::Vec4::from_vec3(&ray.direction, 0.0))),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_create_ray() {
        let ray = Ray::new(
            vec3::Vec3::new(1.0, 3.2, 1.2),
            vec3::Vec3::new(2.0, 2.0, 1.0),
        );
        let origin = ray.origin();
        let direction = ray.direction();

        assert!(math::equal_epsilon_f32(origin.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(origin.y, 3.2, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(origin.z, 1.2, math::EPSILON_F32_5));

        assert!(math::equal_epsilon_f32(
            vec3::Vec3::length(direction),
            1.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            direction.x,
            0.66666,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            direction.y,
            0.66666,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            direction.z,
            0.33333,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_calc_position() {
        let ray = Ray::new(
            vec3::Vec3::new(1.0, 3.2, 1.2),
            vec3::Vec3::new(2.0, 2.0, 1.0),
        );
        let origin = ray.origin();
        let direction = ray.direction();

        let position = ray.calc_position(0.4);
        let mut position_to_origin = position - *origin;
        assert!(math::equal_epsilon_f32(
            vec3::Vec3::length(&position_to_origin),
            0.4 * vec3::Vec3::length(direction),
            math::EPSILON_F32_5
        ));

        position_to_origin = vec3::Vec3::normalize(&position_to_origin).unwrap();
        let normal_direction = vec3::Vec3::normalize(direction).unwrap();
        assert!(math::equal_epsilon_f32(
            vec3::Vec3::dot(&position_to_origin, &normal_direction),
            1.0,
            math::EPSILON_F32_5
        ));
    }
}
