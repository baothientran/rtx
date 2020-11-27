use crate::core::vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &vec3::Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &vec3::Vec3 {
        &self.direction
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
            direction.x,
            2.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            direction.y,
            2.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            direction.z,
            1.0,
            math::EPSILON_F32_5
        ));
    }
}
