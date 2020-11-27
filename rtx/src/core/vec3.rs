use std::ops;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn length_sq(v: &Vec3) -> f32 {
    return v.x * v.x + v.y * v.y + v.z * v.z;
}

pub fn length(v: &Vec3) -> f32 {
    return f32::sqrt(length_sq(v));
}

pub fn normalize(v: &Vec3) -> Option<Vec3> {
    let len = length(v);
    if len == 0.0 {
        return None;
    }

    let inv_len = 1.0 / len;
    return Some(*v * inv_len);
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        return Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        return rhs * self;
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z);
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        return Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = *self / rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_length_sq() {
        let v = Vec3::new(12.2, 21.0, 34.4);
        let expected = v.x * v.x + v.y * v.y + v.z * v.z;
        assert!(math::equal_epsilon_f32(
            expected,
            length_sq(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(12.0, 0.0, 0.0);
        assert!(math::equal_epsilon_f32(
            12.0,
            length(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(12.2, 21.0, 34.4);
        let n = normalize(&v).unwrap();
        assert!(math::equal_epsilon_f32(
            length(&n),
            1.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_add() {
        let lhs = Vec3::new(12.0, 21.0, 34.4);
        let rhs = Vec3::new(23.0, 20.0, 10.0);

        let result = lhs + rhs;
        assert!(math::equal_epsilon_f32(result.x, 35.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 41.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 44.4, math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub() {
        let lhs = Vec3::new(12.0, 21.0, 34.4);
        let rhs = Vec3::new(23.0, 20.0, 10.0);

        let result = lhs - rhs;
        assert!(math::equal_epsilon_f32(
            result.x,
            -11.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 24.4, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul() {
        let lhs = Vec3::new(12.0, 21.0, 34.4);
        let rhs = Vec3::new(23.0, 20.0, 10.0);

        let result = lhs * rhs;
        assert!(math::equal_epsilon_f32(
            result.x,
            276.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.y,
            420.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.z,
            344.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_mul_f32_rhs() {
        let lhs = Vec3::new(12.0, 21.0, 34.4);
        let rhs: f32 = 10.0;

        let result = lhs * rhs;
        assert!(math::equal_epsilon_f32(
            result.x,
            120.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.y,
            210.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.z,
            344.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_mul_f32_lhs() {
        let lhs: f32 = 10.0;
        let rhs = Vec3::new(12.0, 21.0, 34.4);

        let result = lhs * rhs;
        assert!(math::equal_epsilon_f32(
            result.x,
            120.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.y,
            210.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.z,
            344.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_div() {
        let lhs = Vec3::new(12.0, 21.0, 34.4);
        let rhs = Vec3::new(12.0, 10.0, 2.0);

        let result = lhs / rhs;
        assert!(math::equal_epsilon_f32(result.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 17.2, math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_f32_rhs() {
        let lhs = Vec3::new(12.0, 21.0, 34.4);
        let rhs: f32 = 10.0;

        let result = lhs / rhs;
        assert!(math::equal_epsilon_f32(result.x, 1.2, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 3.44, math::EPSILON_F32_5));
    }

    #[test]
    fn test_add_assign() {
        let mut result = Vec3::new(12.0, 21.0, 34.4);
        result += Vec3::new(23.0, 20.0, 10.0);

        assert!(math::equal_epsilon_f32(result.x, 35.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 41.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 44.4, math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub_assign() {
        let mut result = Vec3::new(12.0, 21.0, 34.4);
        result -= Vec3::new(23.0, 20.0, 10.0);

        assert!(math::equal_epsilon_f32(
            result.x,
            -11.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 24.4, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_assign() {
        let mut result = Vec3::new(12.0, 21.0, 34.4);
        result *= Vec3::new(23.0, 20.0, 10.0);

        assert!(math::equal_epsilon_f32(
            result.x,
            276.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.y,
            420.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.z,
            344.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_mul_assign_f32_rhs() {
        let mut result = Vec3::new(12.0, 21.0, 34.4);
        result *= 10.0;

        assert!(math::equal_epsilon_f32(
            result.x,
            120.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.y,
            210.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.z,
            344.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_div_assign() {
        let mut result = Vec3::new(12.0, 21.0, 34.4);
        result /= Vec3::new(12.0, 10.0, 2.0);

        assert!(math::equal_epsilon_f32(result.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 17.2, math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_assign_f32_rhs() {
        let mut result = Vec3::new(12.0, 21.0, 34.4);
        result /= 10.0;

        assert!(math::equal_epsilon_f32(result.x, 1.2, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 3.44, math::EPSILON_F32_5));
    }
}
