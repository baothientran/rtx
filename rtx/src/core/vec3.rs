use crate::core::math;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::convert;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        return Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        );
    }

    pub fn distance(&self, to: &Vec3) -> f32 {
        let direction = to - self;
        return Vec3::length(&direction);
    }

    pub fn length_sq(&self) -> f32 {
        return Vec3::dot(self, self);
    }

    pub fn length(&self) -> f32 {
        return f32::sqrt(Vec3::length_sq(self));
    }

    pub fn normalize(&self) -> Option<Vec3> {
        let len = Vec3::length(self);
        if len == 0.0 {
            return None;
        }

        let inv_len = 1.0 / len;
        return Some(self * inv_len);
    }

    pub fn abs(&self) -> Vec3 {
        return Vec3 {
            x: f32::abs(self.x),
            y: f32::abs(self.y),
            z: f32::abs(self.z),
        };
    }

    pub fn powf(&self, num: f32) -> Vec3 {
        return Vec3 {
            x: f32::powf(self.x, num),
            y: f32::powf(self.y, num),
            z: f32::powf(self.z, num),
        };
    }

    pub fn powi(&self, num: i32) -> Vec3 {
        return Vec3 {
            x: f32::powi(self.x, num),
            y: f32::powi(self.y, num),
            z: f32::powi(self.z, num),
        };
    }

    pub fn sqrt(&self) -> Vec3 {
        return Vec3 {
            x: f32::sqrt(self.x),
            y: f32::sqrt(self.y),
            z: f32::sqrt(self.z),
        };
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        return -self + 2.0 * Vec3::dot(normal, self) * normal;
    }

    pub fn coordinate_system(v1: &Vec3, v2: &mut Vec3, v3: &mut Vec3) {
        if f32::abs(v1.x) > f32::abs(v1.y) {
            *v2 = Vec3::normalize(&Vec3::new(v1.z, 0.0, -v1.x)).unwrap(); // cross(y, v1)
        } else {
            *v2 = Vec3::normalize(&Vec3::new(0.0, -v1.z, v1.y)).unwrap(); // cross(x, v1)
        }

        *v3 = Vec3::cross(v1, v2);
    }

    pub fn equal_epsilon(&self, rhs: &Vec3, epsilon: f32) -> bool {
        return math::equal_epsilon_f32(self.x, rhs.x, epsilon)
            && math::equal_epsilon_f32(self.y, rhs.y, epsilon)
            && math::equal_epsilon_f32(self.z, rhs.z, epsilon);
    }
}

impl convert::From<f32> for Vec3 {
    fn from(num: f32) -> Self {
        return Vec3::new(num, num, num);
    }
}

impl_op_ex!(-|a: &Vec3| -> Vec3 {
    return Vec3 {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    };
});

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    return Vec3::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z);
});

impl_op_ex!(-|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    return Vec3::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z);
});

impl_op_ex!(*|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    return Vec3::new(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z);
});

impl_op_ex_commutative!(*|lhs: &Vec3, rhs: &f32| -> Vec3 {
    return Vec3::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs);
});

impl_op_ex!(/ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    return Vec3::new(lhs.x / rhs.x, lhs.y / rhs.y, lhs.z / rhs.z);
});

impl_op_ex!(/ |lhs: &Vec3, rhs: &f32| -> Vec3 {
    return Vec3::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs);
});

impl_op_ex!(/ |lhs: &f32, rhs: &Vec3| -> Vec3 {
    return Vec3::new(lhs / rhs.x, lhs / rhs.y, lhs / rhs.z);
});

impl_op_ex!(+= |lhs: &mut Vec3, rhs: &Vec3| {
    *lhs = lhs as &Vec3 + rhs;
});

impl_op_ex!(-= |lhs: &mut Vec3, rhs: &Vec3| {
    *lhs = lhs as &Vec3 - rhs;
});

impl_op_ex!(*= |lhs: &mut Vec3, rhs: &Vec3| {
    *lhs = lhs as &Vec3 * rhs;
});

impl_op_ex!(*= |lhs: &mut Vec3, rhs: &f32| {
    *lhs = lhs as &Vec3 * rhs;
});

impl_op_ex!(/= |lhs: &mut Vec3, rhs: &Vec3| {
    *lhs = lhs as &Vec3 / rhs;
});

impl_op_ex!(/= |lhs: &mut Vec3, rhs: &f32| {
    *lhs = lhs as &Vec3 / rhs;
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_cross() {
        let lhs = Vec3::new(12.0, 34.0, 12.0);
        let rhs = Vec3::new(2.0, 1.0, 2.0);
        let result = Vec3::cross(&lhs, &rhs);
        assert!(math::equal_epsilon_f32(
            Vec3::dot(&result, &lhs),
            0.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            Vec3::dot(&result, &rhs),
            0.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(result.x, 56.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 0.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(
            result.z,
            -56.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_dot() {
        let lhs = Vec3::new(12.0, 34.0, 12.0);
        let rhs = Vec3::new(2.0, 1.0, 2.0);
        let result = Vec3::dot(&lhs, &rhs);
        assert!(math::equal_epsilon_f32(result, 82.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_distance() {
        let from = Vec3::new(12.0, 2.0, 3.0);
        let to = Vec3::new(1.0, 2.0, 1.0);
        let dist = Vec3::distance(&from, &to);
        assert!(math::equal_epsilon_f32(dist, 11.18034, math::EPSILON_F32_5));
    }

    #[test]
    fn test_length_sq() {
        let v = Vec3::new(12.2, 21.0, 34.4);
        let expected = v.x * v.x + v.y * v.y + v.z * v.z;
        assert!(math::equal_epsilon_f32(
            expected,
            Vec3::length_sq(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(12.0, 0.0, 0.0);
        assert!(math::equal_epsilon_f32(
            12.0,
            Vec3::length(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(12.2, 21.0, 34.4);
        let n = Vec3::normalize(&v).unwrap();
        assert!(math::equal_epsilon_f32(
            Vec3::length(&n),
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
    fn test_div_f32_lhs() {
        let lhs: f32 = 10.0;
        let rhs = Vec3::new(12.0, 21.0, 34.4);

        let result = lhs / rhs;
        assert!(math::equal_epsilon_f32(
            result.x,
            0.83333,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.y,
            0.47619,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(
            result.z,
            0.29069,
            math::EPSILON_F32_5
        ));
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

    #[test]
    fn test_abs() {
        let v = Vec3::new(-12.0, -1.0, -2.0);
        let result = Vec3::abs(&v);

        assert!(math::equal_epsilon_f32(result.x, 12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 2.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_powf() {
        let v = Vec3::new(4.0, 9.0, 16.0);
        let result = Vec3::powf(&v, 2.0);
        assert!(Vec3::equal_epsilon(
            &result,
            &Vec3::new(16.0, 81.0, 256.0),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_powi() {
        let v = Vec3::new(4.0, 9.0, 16.0);
        let result = Vec3::powi(&v, 2);
        assert!(Vec3::equal_epsilon(
            &result,
            &Vec3::new(16.0, 81.0, 256.0),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_sqrt() {
        let v = Vec3::new(4.0, 9.0, 16.0);
        let result = Vec3::sqrt(&v);

        assert!(math::equal_epsilon_f32(result.x, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 3.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 4.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_from_f32() {
        let v = Vec3::from(-12.0);
        assert!(math::equal_epsilon_f32(v.x, -12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(v.y, -12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(v.z, -12.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(-12.0, -1.0, -2.0);
        let result = -v;

        assert!(math::equal_epsilon_f32(result.x, 12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 2.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_reflect() {
        let v = Vec3::normalize(&Vec3::new(1.0, 1.0, 0.0)).unwrap();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let reflect = Vec3::reflect(&v, &n);
        assert!(Vec3::equal_epsilon(
            &reflect,
            &Vec3::normalize(&Vec3::new(-1.0, 1.0, 0.0)).unwrap(),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_coordinate_system() {
        let y_axis = Vec3::normalize(&Vec3::new(1.0, 0.0, 1.0)).unwrap();
        let mut z_axis = Vec3::from(0.0);
        let mut x_axis = Vec3::from(0.0);
        Vec3::coordinate_system(&y_axis, &mut z_axis, &mut x_axis);
        assert!(Vec3::equal_epsilon(
            &Vec3::cross(&x_axis, &y_axis),
            &z_axis,
            math::EPSILON_F32_5
        ));
        assert!(Vec3::equal_epsilon(
            &Vec3::cross(&y_axis, &z_axis),
            &x_axis,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_equal_epsilon() {
        let lhs = Vec3::new(2.000002, 3.000004, 2.000001);
        let rhs = Vec3::new(2.000001, 3.000003, 2.000001);
        assert!(Vec3::equal_epsilon(&lhs, &rhs, math::EPSILON_F32_5));
    }

    #[test]
    fn test_not_equal_epsilon() {
        let lhs = Vec3::new(2.000002, 3.000004, 2.000001);
        let rhs = Vec3::new(2.00002, 3.00003, 2.000001);
        assert!(!Vec3::equal_epsilon(&lhs, &rhs, math::EPSILON_F32_5));
    }
}
