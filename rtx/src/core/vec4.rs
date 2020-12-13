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

    pub fn equal_epsilon(lhs: &Vec4, rhs: &Vec4, epsilon: f32) -> bool {
        return math::equal_epsilon_f32(lhs.x, rhs.x, epsilon) && 
               math::equal_epsilon_f32(lhs.y, rhs.y, epsilon) && 
               math::equal_epsilon_f32(lhs.z, rhs.z, epsilon) &&
               math::equal_epsilon_f32(lhs.w, rhs.w, epsilon);
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
