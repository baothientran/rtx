use crate::core::math;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::convert;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        return Vec2 { x, y };
    }

    pub fn dot(&self, v: &Vec2) -> f32 {
        return self.x * v.x + self.y * v.y;
    }

    pub fn distance(&self, to: &Vec2) -> f32 {
        let direction = to - self;
        return Vec2::length(&direction);
    }

    pub fn length_sq(&self) -> f32 {
        return Vec2::dot(self, self);
    }

    pub fn length(&self) -> f32 {
        return f32::sqrt(Vec2::length_sq(self));
    }

    pub fn normalize(&self) -> Option<Vec2> {
        let len = Vec2::length(self);
        if len == 0.0 {
            return None;
        }

        let inv_len = 1.0 / len;
        return Some(self * inv_len);
    }

    pub fn abs(&self) -> Vec2 {
        return Vec2 {
            x: f32::abs(self.x),
            y: f32::abs(self.y),
        };
    }

    pub fn powf(&self, num: f32) -> Vec2 {
        return Vec2 {
            x: f32::powf(self.x, num),
            y: f32::powf(self.y, num),
        };
    }

    pub fn powi(&self, num: i32) -> Vec2 {
        return Vec2 {
            x: f32::powi(self.x, num),
            y: f32::powi(self.y, num),
        };
    }

    pub fn sqrt(&self) -> Vec2 {
        return Vec2 {
            x: f32::sqrt(self.x),
            y: f32::sqrt(self.y),
        };
    }

    pub fn reflect(&self, normal: &Vec2) -> Vec2 {
        return -self + 2.0 * Vec2::dot(normal, self) * normal;
    }

    pub fn equal_epsilon(&self, rhs: &Vec2, epsilon: f32) -> bool {
        return math::equal_epsilon_f32(self.x, rhs.x, epsilon)
            && math::equal_epsilon_f32(self.y, rhs.y, epsilon);
    }
}

impl convert::From<f32> for Vec2 {
    fn from(num: f32) -> Self {
        return Vec2::new(num, num);
    }
}

impl_op_ex!(-|a: &Vec2| -> Vec2 {
    return Vec2 { x: -a.x, y: -a.y };
});

impl_op_ex!(+ |lhs: &Vec2, rhs: &Vec2| -> Vec2 {
    return Vec2::new(lhs.x + rhs.x, lhs.y + rhs.y);
});

impl_op_ex!(-|lhs: &Vec2, rhs: &Vec2| -> Vec2 {
    return Vec2::new(lhs.x - rhs.x, lhs.y - rhs.y);
});

impl_op_ex!(*|lhs: &Vec2, rhs: &Vec2| -> Vec2 {
    return Vec2::new(lhs.x * rhs.x, lhs.y * rhs.y);
});

impl_op_ex_commutative!(*|lhs: &Vec2, rhs: &f32| -> Vec2 {
    return Vec2::new(lhs.x * rhs, lhs.y * rhs);
});

impl_op_ex!(/ |lhs: &Vec2, rhs: &Vec2| -> Vec2 {
    return Vec2::new(lhs.x / rhs.x, lhs.y / rhs.y);
});

impl_op_ex!(/ |lhs: &Vec2, rhs: &f32| -> Vec2 {
    return Vec2::new(lhs.x / rhs, lhs.y / rhs);
});

impl_op_ex!(/ |lhs: &f32, rhs: &Vec2| -> Vec2 {
    return Vec2::new(lhs / rhs.x, lhs / rhs.y);
});

impl_op_ex!(+= |lhs: &mut Vec2, rhs: &Vec2| {
    *lhs = lhs as &Vec2 + rhs;
});

impl_op_ex!(-= |lhs: &mut Vec2, rhs: &Vec2| {
    *lhs = lhs as &Vec2 - rhs;
});

impl_op_ex!(*= |lhs: &mut Vec2, rhs: &Vec2| {
    *lhs = lhs as &Vec2 * rhs;
});

impl_op_ex!(*= |lhs: &mut Vec2, rhs: &f32| {
    *lhs = lhs as &Vec2 * rhs;
});

impl_op_ex!(/= |lhs: &mut Vec2, rhs: &Vec2| {
    *lhs = lhs as &Vec2 / rhs;
});

impl_op_ex!(/= |lhs: &mut Vec2, rhs: &f32| {
    *lhs = lhs as &Vec2 / rhs;
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_dot() {
        let lhs = Vec2::new(12.0, 34.0);
        let rhs = Vec2::new(2.0, 1.0);
        let result = Vec2::dot(&lhs, &rhs);
        assert!(math::equal_epsilon_f32(result, 58.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_distance() {
        let from = Vec2::new(12.0, 2.0);
        let to = Vec2::new(1.0, 2.0);
        let dist = Vec2::distance(&from, &to);
        assert!(math::equal_epsilon_f32(dist, 11.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_length_sq() {
        let v = Vec2::new(12.2, 21.0);
        let expected = v.x * v.x + v.y * v.y;
        assert!(math::equal_epsilon_f32(
            expected,
            Vec2::length_sq(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_length() {
        let v = Vec2::new(12.0, 0.0);
        assert!(math::equal_epsilon_f32(
            12.0,
            Vec2::length(&v),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_normalize() {
        let v = Vec2::new(12.2, 21.0);
        let n = Vec2::normalize(&v).unwrap();
        assert!(math::equal_epsilon_f32(
            Vec2::length(&n),
            1.0,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_add() {
        let lhs = Vec2::new(12.0, 21.0);
        let rhs = Vec2::new(23.0, 20.0);

        let result = lhs + rhs;
        assert!(math::equal_epsilon_f32(result.x, 35.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 41.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub() {
        let lhs = Vec2::new(12.0, 21.0);
        let rhs = Vec2::new(23.0, 20.0);

        let result = lhs - rhs;
        assert!(math::equal_epsilon_f32(
            result.x,
            -11.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul() {
        let lhs = Vec2::new(12.0, 21.0);
        let rhs = Vec2::new(23.0, 20.0);

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
    }

    #[test]
    fn test_mul_f32_rhs() {
        let lhs = Vec2::new(12.0, 21.0);
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
    }

    #[test]
    fn test_mul_f32_lhs() {
        let lhs: f32 = 10.0;
        let rhs = Vec2::new(12.0, 21.0);

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
    }

    #[test]
    fn test_div() {
        let lhs = Vec2::new(12.0, 21.0);
        let rhs = Vec2::new(12.0, 10.0);

        let result = lhs / rhs;
        assert!(math::equal_epsilon_f32(result.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_f32_rhs() {
        let lhs = Vec2::new(12.0, 21.0);
        let rhs: f32 = 10.0;

        let result = lhs / rhs;
        assert!(math::equal_epsilon_f32(result.x, 1.2, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_f32_lhs() {
        let lhs: f32 = 10.0;
        let rhs = Vec2::new(12.0, 21.0);

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
    }

    #[test]
    fn test_add_assign() {
        let mut result = Vec2::new(12.0, 21.0);
        result += Vec2::new(23.0, 20.0);

        assert!(math::equal_epsilon_f32(result.x, 35.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 41.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub_assign() {
        let mut result = Vec2::new(12.0, 21.0);
        result -= Vec2::new(23.0, 20.0);

        assert!(math::equal_epsilon_f32(
            result.x,
            -11.0,
            math::EPSILON_F32_5
        ));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_assign() {
        let mut result = Vec2::new(12.0, 21.0);
        result *= Vec2::new(23.0, 20.0);

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
    }

    #[test]
    fn test_mul_assign_f32_rhs() {
        let mut result = Vec2::new(12.0, 21.0);
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
    }

    #[test]
    fn test_div_assign() {
        let mut result = Vec2::new(12.0, 21.0);
        result /= Vec2::new(12.0, 10.0);

        assert!(math::equal_epsilon_f32(result.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_assign_f32_rhs() {
        let mut result = Vec2::new(12.0, 21.0);
        result /= 10.0;

        assert!(math::equal_epsilon_f32(result.x, 1.2, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 2.1, math::EPSILON_F32_5));
    }

    #[test]
    fn test_abs() {
        let v = Vec2::new(-12.0, -1.0);
        let result = Vec2::abs(&v);

        assert!(math::equal_epsilon_f32(result.x, 12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_powf() {
        let v = Vec2::new(4.0, 9.0);
        let result = Vec2::powf(&v, 2.0);
        assert!(Vec2::equal_epsilon(
            &result,
            &Vec2::new(16.0, 81.0),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_powi() {
        let v = Vec2::new(4.0, 9.0);
        let result = Vec2::powi(&v, 2);
        assert!(Vec2::equal_epsilon(
            &result,
            &Vec2::new(16.0, 81.0),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_sqrt() {
        let v = Vec2::new(4.0, 9.0);
        let result = Vec2::sqrt(&v);

        assert!(math::equal_epsilon_f32(result.x, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 3.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_from_f32() {
        let v = Vec2::from(-12.0);
        assert!(math::equal_epsilon_f32(v.x, -12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(v.y, -12.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_neg() {
        let v = Vec2::new(-12.0, -1.0);
        let result = -v;

        assert!(math::equal_epsilon_f32(result.x, 12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 1.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_reflect() {
        let v = Vec2::normalize(&Vec2::new(1.0, 1.0)).unwrap();
        let n = Vec2::new(0.0, 1.0);
        let reflect = Vec2::reflect(&v, &n);
        assert!(Vec2::equal_epsilon(
            &reflect,
            &Vec2::normalize(&Vec2::new(-1.0, 1.0)).unwrap(),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_equal_epsilon() {
        let lhs = Vec2::new(2.000002, 3.000004);
        let rhs = Vec2::new(2.000001, 3.000003);
        assert!(Vec2::equal_epsilon(&lhs, &rhs, math::EPSILON_F32_5));
    }

    #[test]
    fn test_not_equal_epsilon() {
        let lhs = Vec2::new(2.000002, 3.000004);
        let rhs = Vec2::new(2.00002, 3.00003);
        assert!(!Vec2::equal_epsilon(&lhs, &rhs, math::EPSILON_F32_5));
    }
}
