use crate::core::vec3;

pub struct AABB {
    min: vec3::Vec3,
    max: vec3::Vec3,
}

impl AABB {
    pub fn new(min: vec3::Vec3, max: vec3::Vec3) -> AABB {
        return AABB { min, max };
    }

    pub fn min(&self) -> &vec3::Vec3 {
        return &self.min;
    }

    pub fn max(&self) -> &vec3::Vec3 {
        return &self.max;
    }

    pub fn center(&self) -> vec3::Vec3 {
        return (self.min + self.max) / 2.0;
    }

    pub fn merge(&mut self, v: &vec3::Vec3) {
        self.min.x = f32::min(self.min.x, v.x);
        self.min.y = f32::min(self.min.y, v.y);
        self.min.z = f32::min(self.min.z, v.z);

        self.max.x = f32::max(self.max.x, v.x);
        self.max.y = f32::max(self.max.y, v.y);
        self.max.z = f32::max(self.max.z, v.z);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_create_aabb() {
        let aabb = AABB::new(
            vec3::Vec3::new(1.0, 2.0, 3.0),
            vec3::Vec3::new(2.0, 4.0, 5.0),
        );
        let min = aabb.min();
        let max = aabb.max();
        let center = aabb.center();
        assert!(math::equal_epsilon_f32(min.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(min.y, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(min.z, 3.0, math::EPSILON_F32_5));

        assert!(math::equal_epsilon_f32(max.x, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(max.y, 4.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(max.z, 5.0, math::EPSILON_F32_5));

        assert!(math::equal_epsilon_f32(center.x, 1.5, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.y, 3.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.z, 4.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_merge() {
        let mut aabb = AABB::new(
            vec3::Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            vec3::Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        );
        let v0 = vec3::Vec3::new(1.0, 4.0, 3.0);
        let v1 = vec3::Vec3::new(2.0, 2.0, 5.0);
        aabb.merge(&v0);
        aabb.merge(&v1);

        let min = aabb.min();
        let max = aabb.max();
        let center = aabb.center();
        assert!(math::equal_epsilon_f32(min.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(min.y, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(min.z, 3.0, math::EPSILON_F32_5));

        assert!(math::equal_epsilon_f32(max.x, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(max.y, 4.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(max.z, 5.0, math::EPSILON_F32_5));

        assert!(math::equal_epsilon_f32(center.x, 1.5, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.y, 3.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.z, 4.0, math::EPSILON_F32_5));
    }
}
