use crate::core::math;
use crate::core::vec3;
use std::convert;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        return Vec4 { x, y, z, w };
    }

    pub fn from_vec3(v: &vec3::Vec3, w: f32) -> Vec4 {
        return Vec4::new(v.x, v.y, v.z, w);
    }

    pub fn to_vec3(v: &Vec4) -> vec3::Vec3 {
        return vec3::Vec3::new(v.x, v.y, v.z);
    }

    pub fn dot(lhs: &Vec4, rhs: &Vec4) -> f32 {
        return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w;
    }

    pub fn distance(from: &Vec4, to: &Vec4) -> f32 {
        let direction = *to - *from;
        return Vec4::length(&direction);
    }

    pub fn length_sq(v: &Vec4) -> f32 {
        return Vec4::dot(v, v);
    }

    pub fn length(v: &Vec4) -> f32 {
        return f32::sqrt(Vec4::length_sq(v));
    }

    pub fn normalize(v: &Vec4) -> Option<Vec4> {
        let len = Vec4::length(v);
        if len == 0.0 {
            return None;
        }

        let inv_len = 1.0 / len;
        return Some(*v * inv_len);
    }

    pub fn abs(v: &Vec4) -> Vec4 {
        return Vec4 {
            x: f32::abs(v.x),
            y: f32::abs(v.y),
            z: f32::abs(v.z),
            w: f32::abs(v.w),
        };
    }

    pub fn sqrt(v: &Vec4) -> Vec4 {
        return Vec4 {
            x: f32::sqrt(v.x),
            y: f32::sqrt(v.y),
            z: f32::sqrt(v.z),
            w: f32::sqrt(v.w),
        };
    }

    pub fn equal_epsilon(lhs: &Vec4, rhs: &Vec4, epsilon: f32) -> bool {
        return math::equal_epsilon_f32(lhs.x, rhs.x, epsilon)
            && math::equal_epsilon_f32(lhs.y, rhs.y, epsilon)
            && math::equal_epsilon_f32(lhs.z, rhs.z, epsilon)
            && math::equal_epsilon_f32(lhs.w, rhs.w, epsilon);
    }
}

impl convert::From<f32> for Vec4 {
    fn from(num: f32) -> Self {
        return Vec4::new(num, num, num, num);
    }
}

impl ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self {
        return Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        };
    }
}

impl ops::Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Vec4 {
        return Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        );
    }
}

impl ops::Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Vec4 {
        return Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        );
    }
}

impl ops::Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        return Vec4::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        );
    }
}

impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Vec4 {
        return Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs);
    }
}

impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        return rhs * self;
    }
}

impl ops::Div for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Vec4 {
        return Vec4::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        );
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Vec4 {
        return Vec4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs);
    }
}

impl ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Vec4) {
        *self = *self * rhs;
    }
}

impl ops::MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Vec4) {
        *self = *self / rhs;
    }
}

impl ops::DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_dot() {
        let lhs = Vec4::new(12.0, 34.0, 12.0, 2.0);
        let rhs = Vec4::new(2.0, 1.0, 2.0, 1.0);
        let result = Vec4::dot(&lhs, &rhs);
        assert!(math::equal_epsilon_f32(result, 84.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_distance() {
        let from = Vec4::new(12.0, 2.0, 3.0, 1.0);
        let to = Vec4::new(1.0, 2.0, 1.0, 2.0);
        let dist = Vec4::distance(&from, &to);
        assert!(math::equal_epsilon_f32(dist, 11.224972, math::EPSILON_F32_5));
    }

    #[test]
    fn test_length_sq() {
        let v = Vec4::new(12.2, 21.0, 34.4, 2.0);
        let expected = v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w;
        assert!(math::equal_epsilon_f32(
            expected,
            Vec4::length_sq(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_length() {
        let v = Vec4::new(12.0, 0.0, 0.0, 2.0);
        assert!(math::equal_epsilon_f32(
            12.165525,
            Vec4::length(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_normalize() {
        let v = Vec4::new(12.2, 21.0, 34.4, 2.0);
        let n = Vec4::normalize(&v).unwrap();
        assert!(math::equal_epsilon_f32(
            Vec4::length(&n),
            1.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_add() {
        let lhs = Vec4::new(12.0, 21.0, 34.4, 2.0);
        let rhs = Vec4::new(23.0, 20.0, 10.0, 1.0);

        let result = lhs + rhs;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(35.0, 41.0, 44.4, 3.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub() {
        let lhs = Vec4::new(12.0, 21.0, 34.4, 2.0);
        let rhs = Vec4::new(23.0, 20.0, 10.0, 3.0);

        let result = lhs - rhs;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(-11.0, 1.0, 24.4, -1.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul() {
        let lhs = Vec4::new(12.0, 21.0, 34.4, 2.0);
        let rhs = Vec4::new(23.0, 20.0, 10.0, 21.0);

        let result = lhs * rhs;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(276.0, 420.0, 344.0, 42.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_f32_rhs() {
        let lhs = Vec4::new(12.0, 21.0, 34.4, 2.0);
        let rhs: f32 = 10.0;

        let result = lhs * rhs;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(120.0, 210.0, 344.0, 20.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_f32_lhs() {
        let lhs: f32 = 10.0;
        let rhs = Vec4::new(12.0, 21.0, 34.4, 2.1);

        let result = lhs * rhs;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(120.0, 210.0, 344.0, 21.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_div() {
        let lhs = Vec4::new(12.0, 21.0, 34.4, 1.0);
        let rhs = Vec4::new(12.0, 10.0, 2.0, 2.0);

        let result = lhs / rhs;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(1.0, 2.1, 17.2, 0.5), math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_f32_rhs() {
        let lhs = Vec4::new(12.0, 21.0, 34.4, 1.0);
        let rhs: f32 = 10.0;

        let result = lhs / rhs;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(1.2, 2.1, 3.44, 0.1), math::EPSILON_F32_5));
    }

    #[test]
    fn test_add_assign() {
        let mut result = Vec4::new(12.0, 21.0, 34.4, 2.0);
        result += Vec4::new(23.0, 20.0, 10.0, 2.0);
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(35.0, 41.0, 44.4, 4.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub_assign() {
        let mut result = Vec4::new(12.0, 21.0, 34.4, 5.0);
        result -= Vec4::new(23.0, 20.0, 10.0, 1.0);
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(-11.0, 1.0, 24.4, 4.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_assign() {
        let mut result = Vec4::new(12.0, 21.0, 34.4, 2.0);
        result *= Vec4::new(23.0, 20.0, 10.0, 1.0);
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(276.0, 420.0, 344.0, 2.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_assign_f32_rhs() {
        let mut result = Vec4::new(12.0, 21.0, 34.4, 1.0);
        result *= 10.0;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(120.0, 210.0, 344.0, 10.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_assign() {
        let mut result = Vec4::new(12.0, 21.0, 34.4, 1.0);
        result /= Vec4::new(12.0, 10.0, 2.0, 2.0);
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(1.0, 2.1, 17.2, 0.5), math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_assign_f32_rhs() {
        let mut result = Vec4::new(12.0, 21.0, 34.4, 20.0);
        result /= 10.0;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(1.2, 2.1, 3.44, 2.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_abs() {
        let v = Vec4::new(-12.0, -1.0, -2.0, -0.5);
        let result = Vec4::abs(&v);
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(12.0, 1.0, 2.0, 0.5), math::EPSILON_F32_5));
    }

    #[test]
    fn test_sqrt() {
        let v = Vec4::new(4.0, 9.0, 16.0, 9.0);
        let result = Vec4::sqrt(&v);
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(2.0, 3.0, 4.0, 3.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_from_f32() {
        let v = Vec4::from(-12.0);
        assert!(Vec4::equal_epsilon(&v, &Vec4::new(-12.0, -12.0, -12.0, -12.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_neg() {
        let v = Vec4::new(-12.0, -1.0, -2.0, -1.0);
        let result = -v;
        assert!(Vec4::equal_epsilon(&result, &Vec4::new(12.0, 1.0, 2.0, 1.0), math::EPSILON_F32_5));
    }

    #[test]
    fn test_equal_epsilon() {
        let lhs = Vec4::new(2.000002, 3.000004, 2.000001, 1.000001);
        let rhs = Vec4::new(2.000001, 3.000003, 2.000001, 1.000001);
        assert!(Vec4::equal_epsilon(&lhs, &rhs, math::EPSILON_F32_5));
    }

    #[test]
    fn test_not_equal_epsilon() {
        let lhs = Vec4::new(2.000002, 3.000004, 2.000001, 1.0);
        let rhs = Vec4::new(2.00002, 3.00003, 2.000001, 1.0);
        assert!(!Vec4::equal_epsilon(&lhs, &rhs, math::EPSILON_F32_5));
    }
}
