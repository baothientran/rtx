use std::cmp::{max, min};
use std::ops;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

impl Color {
    pub fn new(red: u8, blue: u8, green: u8) -> Color {
        Color { red, blue, green }
    }
}

// Color + Color
impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self {
        let red = self.red as u16 + rhs.red as u16;
        let red = min(red, 255) as u8;

        let blue = self.blue as u16 + rhs.blue as u16;
        let blue = min(blue, 255) as u8;

        let green = self.green as u16 + rhs.green as u16;
        let green = min(green, 255) as u8;

        Color { red, blue, green }
    }
}

// Color - Color
impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self {
        let red = self.red as i16 - rhs.red as i16;
        let red = max(red, 0) as u8;

        let blue = self.blue as i16 - rhs.blue as i16;
        let blue = max(blue, 0) as u8;

        let green = self.green as i16 - rhs.green as i16;
        let green = max(green, 0) as u8;

        Color { red, blue, green }
    }
}

// Color * Color
impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self {
        let red = self.red as u16 * rhs.red as u16;
        let red = min(red, 255) as u8;

        let blue = self.blue as u16 * rhs.blue as u16;
        let blue = min(blue, 255) as u8;

        let green = self.green as u16 * rhs.green as u16;
        let green = min(green, 255) as u8;

        Color { red, blue, green }
    }
}

// Color * i8
impl ops::Mul<i8> for Color {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self {
        let red = self.red as i16 * rhs as i16;
        let red = max(min(red, 255), 0) as u8;

        let blue = self.blue as i16 * rhs as i16;
        let blue = max(min(blue, 255), 0) as u8;

        let green = self.green as i16 * rhs as i16;
        let green = max(min(green, 255), 0) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for i8 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * u8
impl ops::Mul<u8> for Color {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self {
        let red = self.red as u16 * rhs as u16;
        let red = min(red, 255) as u8;

        let blue = self.blue as u16 * rhs as u16;
        let blue = min(blue, 255) as u8;

        let green = self.green as u16 * rhs as u16;
        let green = min(green, 255) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for u8 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * i16
impl ops::Mul<i16> for Color {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self {
        let red = self.red as i16 * rhs;
        let red = max(min(red, 255), 0) as u8;

        let blue = self.blue as i16 * rhs;
        let blue = max(min(blue, 255), 0) as u8;

        let green = self.green as i16 * rhs;
        let green = max(min(green, 255), 0) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for i16 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * u16
impl ops::Mul<u16> for Color {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self {
        let red = self.red as u16 * rhs;
        let red = min(red, 255) as u8;

        let blue = self.blue as u16 * rhs;
        let blue = min(blue, 255) as u8;

        let green = self.green as u16 * rhs;
        let green = min(green, 255) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for u16 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * i32
impl ops::Mul<i32> for Color {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        let red = self.red as i32 * rhs;
        let red = max(min(red, 255), 0) as u8;

        let blue = self.blue as i32 * rhs;
        let blue = max(min(blue, 255), 0) as u8;

        let green = self.green as i32 * rhs;
        let green = max(min(green, 255), 0) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for i32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * u32
impl ops::Mul<u32> for Color {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        let red = self.red as u32 * rhs;
        let red = min(red, 255) as u8;

        let blue = self.blue as u32 * rhs;
        let blue = min(blue, 255) as u8;

        let green = self.green as u32 * rhs;
        let green = min(green, 255) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for u32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * i64
impl ops::Mul<i64> for Color {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        let red = self.red as i64 * rhs;
        let red = max(min(red, 255), 0) as u8;

        let blue = self.blue as i64 * rhs;
        let blue = max(min(blue, 255), 0) as u8;

        let green = self.green as i64 * rhs;
        let green = max(min(green, 255), 0) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for i64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * u64
impl ops::Mul<u64> for Color {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        let red = self.red as u64 * rhs;
        let red = min(red, 255) as u8;

        let blue = self.blue as u64 * rhs;
        let blue = min(blue, 255) as u8;

        let green = self.green as u64 * rhs;
        let green = min(green, 255) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for u64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * f32
impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let red = self.red as f32 * rhs;
        let red = f32::max(f32::min(red, 255.0), 0.0) as u8;

        let blue = self.blue as f32 * rhs;
        let blue = f32::max(f32::min(blue, 255.0), 0.0) as u8;

        let green = self.green as f32 * rhs;
        let green = f32::max(f32::min(green, 255.0), 0.0) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color * f64
impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let red = self.red as f64 * rhs;
        let red = f64::max(f64::min(red, 255.0), 0.0) as u8;

        let blue = self.blue as f64 * rhs;
        let blue = f64::max(f64::min(blue, 255.0), 0.0) as u8;

        let green = self.green as f64 * rhs;
        let green = f64::max(f64::min(green, 255.0), 0.0) as u8;

        Color { red, blue, green }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

// Color += Color
impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}

// Color -= Color
impl ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        *self = *self - rhs;
    }
}

// Color *= Color
impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Color) {
        *self = *self * rhs;
    }
}

// Color *= i8;
impl ops::MulAssign<i8> for Color {
    fn mul_assign(&mut self, rhs: i8) {
        *self = *self * rhs;
    }
}

// Color *= u8
impl ops::MulAssign<u8> for Color {
    fn mul_assign(&mut self, rhs: u8) {
        *self = *self * rhs;
    }
}

// Color *= i16
impl ops::MulAssign<i16> for Color {
    fn mul_assign(&mut self, rhs: i16) {
        *self = *self * rhs;
    }
}

// Color *= u16
impl ops::MulAssign<u16> for Color {
    fn mul_assign(&mut self, rhs: u16) {
        *self = *self * rhs;
    }
}

// Color *= i32
impl ops::MulAssign<i32> for Color {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

// Color *= u32
impl ops::MulAssign<u32> for Color {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

// Color *= i64
impl ops::MulAssign<i64> for Color {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

// Color *= u64
impl ops::MulAssign<u64> for Color {
    fn mul_assign(&mut self, rhs: u64) {
        *self = *self * rhs;
    }
}

// Color *= f32
impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

// Color *= f64
impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

// Unit tests for color
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create() {
        let color = Color::new(12, 23, 43);
        assert_eq!(color.red, 12);
        assert_eq!(color.blue, 23);
        assert_eq!(color.green, 43);
    }

    #[test]
    fn test_add() {
        let lhs = Color {
            red: 12,
            blue: 23,
            green: 12,
        };

        let rhs = Color {
            red: 12,
            blue: 23,
            green: 23,
        };

        let expected = Color {
            red: 24,
            blue: 46,
            green: 35,
        };

        assert_eq!(lhs + rhs, expected);
    }

    #[test]
    fn test_add_overflow() {
        let lhs = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let rhs = Color {
            red: 245,
            blue: 250,
            green: 255,
        };

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs + rhs, expected);
    }

    #[test]
    fn test_sub() {
        let lhs = Color {
            red: 23,
            blue: 22,
            green: 34,
        };

        let rhs = Color {
            red: 12,
            blue: 10,
            green: 14,
        };

        let expected = Color {
            red: 11,
            blue: 12,
            green: 20,
        };

        assert_eq!(lhs - rhs, expected);
    }

    #[test]
    fn test_sub_negative() {
        let lhs = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let rhs = Color {
            red: 23,
            blue: 23,
            green: 23,
        };

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(lhs - rhs, expected);
    }

    #[test]
    fn test_mul_color() {
        let lhs = Color {
            red: 12,
            blue: 12,
            green: 24,
        };

        let rhs = Color {
            red: 12,
            blue: 12,
            green: 1,
        };

        let expected = Color {
            red: 144,
            blue: 144,
            green: 24,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_color_overflow() {
        let lhs = Color {
            red: 12,
            blue: 12,
            green: 24,
        };

        let rhs = Color {
            red: 22,
            blue: 22,
            green: 24,
        };

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i8_rhs() {
        let lhs = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let rhs: i8 = 12;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i8_rhs_overflow() {
        let lhs = Color {
            red: 244,
            blue: 244,
            green: 244,
        };

        let rhs: i8 = 127;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i8_rhs_negative() {
        let lhs = Color {
            red: 244,
            blue: 244,
            green: 244,
        };

        let rhs: i8 = -127;

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i8_lhs() {
        let lhs: i8 = 12;

        let rhs = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u8_rhs() {
        let lhs = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let rhs: u8 = 12;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u8_rhs_overflow() {
        let lhs = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let rhs: u8 = 255;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u8_lhs() {
        let lhs: u8 = 12;

        let rhs = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i16_rhs() {
        let lhs = Color {
            red: 12,
            blue: 32,
            green: 24,
        };

        let rhs: i16 = 12;

        let expected = Color {
            red: 144,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i16_rhs_overflow() {
        let lhs = Color {
            red: 12,
            blue: 32,
            green: 24,
        };

        let rhs: i16 = 500;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i16_rhs_negative() {
        let lhs = Color {
            red: 12,
            blue: 32,
            green: 24,
        };

        let rhs: i16 = -500;

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i16_lhs() {
        let lhs: i16 = 2;

        let rhs = Color {
            red: 23,
            blue: 32,
            green: 24,
        };

        let expected = Color {
            red: 46,
            blue: 64,
            green: 48,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u16_rhs() {
        let lhs = Color {
            red: 12,
            blue: 32,
            green: 24,
        };

        let rhs: u16 = 5;

        let expected = Color {
            red: 60,
            blue: 160,
            green: 120,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u16_rhs_overflow() {
        let lhs = Color {
            red: 12,
            blue: 32,
            green: 24,
        };

        let rhs: u16 = 500;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u16_lhs() {
        let lhs: u16 = 10;

        let rhs = Color {
            red: 12,
            blue: 23,
            green: 24,
        };

        let expected = Color {
            red: 120,
            blue: 230,
            green: 240,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i32_rhs() {
        let lhs = Color {
            red: 12,
            blue: 23,
            green: 34,
        };

        let rhs: i32 = 12;

        let expected = Color {
            red: 144,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i32_rhs_overflow() {
        let lhs = Color {
            red: 23,
            blue: 32,
            green: 64,
        };

        let rhs: i32 = 344;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i32_rhs_negative() {
        let lhs = Color {
            red: 34,
            blue: 23,
            green: 64,
        };

        let rhs: i32 = -12;

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(lhs * rhs, expected)
    }

    #[test]
    fn test_mul_i32_lhs() {
        let lhs: i32 = 12;

        let rhs = Color {
            red: 12,
            blue: 23,
            green: 32,
        };

        let expected = Color {
            red: 144,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u32_rhs() {
        let lhs = Color {
            red: 23,
            blue: 32,
            green: 64,
        };

        let rhs: u32 = 10;

        let expected = Color {
            red: 230,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u32_rhs_overflow() {
        let lhs = Color {
            red: 23,
            blue: 32,
            green: 64,
        };

        let rhs: u32 = 100;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u32_lhs() {
        let lhs: u32 = 10;

        let rhs = Color {
            red: 12,
            blue: 10,
            green: 8,
        };

        let expected = Color {
            red: 120,
            blue: 100,
            green: 80,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i64_rhs() {
        let lhs = Color {
            red: 23,
            blue: 32,
            green: 8,
        };

        let rhs: i64 = 2;

        let expected = Color {
            red: 46,
            blue: 64,
            green: 16,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i64_rhs_overflow() {
        let lhs = Color {
            red: 23,
            blue: 32,
            green: 32,
        };

        let rhs: i64 = 30;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i64_rhs_negative() {
        let lhs = Color {
            red: 23,
            blue: 34,
            green: 33,
        };

        let rhs: i64 = -12;

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_i64_lhs() {
        let lhs: i64 = 2;

        let rhs = Color {
            red: 32,
            blue: 64,
            green: 32,
        };

        let expected = Color {
            red: 64,
            blue: 128,
            green: 64,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u64_rhs() {
        let lhs = Color {
            red: 32,
            blue: 64,
            green: 53,
        };

        let rhs: u64 = 2;

        let expected = Color {
            red: 64,
            blue: 128,
            green: 106,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u64_rhs_overflow() {
        let lhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let rhs: u64 = 255;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_u64_lhs() {
        let lhs: u64 = 2;

        let rhs = Color {
            red: 25,
            blue: 23,
            green: 12,
        };

        let expected = Color {
            red: 50,
            blue: 46,
            green: 24,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f32_rhs() {
        let lhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let rhs: f32 = 2.5;

        let expected = Color {
            red: 135,
            blue: 30,
            green: 30,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f32_rhs_overflow() {
        let lhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let rhs: f32 = 112.5;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f32_rhs_negative() {
        let lhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let rhs: f32 = -112.5;

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f32_lhs() {
        let lhs: f32 = 2.5;

        let rhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let expected = Color {
            red: 135,
            blue: 30,
            green: 30,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f64_rhs() {
        let lhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let rhs: f64 = 2.5;

        let expected = Color {
            red: 135,
            blue: 30,
            green: 30,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f64_rhs_overflow() {
        let lhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let rhs: f64 = 112.5;

        let expected = Color {
            red: 255,
            blue: 255,
            green: 255,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f64_rhs_negative() {
        let lhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let rhs: f64 = -112.5;

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_mul_f64_lhs() {
        let lhs: f64 = 2.5;

        let rhs = Color {
            red: 54,
            blue: 12,
            green: 12,
        };

        let expected = Color {
            red: 135,
            blue: 30,
            green: 30,
        };

        assert_eq!(lhs * rhs, expected);
    }

    #[test]
    fn test_add_assign() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let add = Color {
            red: 244,
            blue: 12,
            green: 23,
        };

        color += add;

        let expected = Color {
            red: 255,
            blue: 24,
            green: 35,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_sub_assign() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let sub = Color {
            red: 244,
            blue: 12,
            green: 23,
        };

        color -= sub;

        let expected = Color {
            red: 0,
            blue: 0,
            green: 0,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul = Color {
            red: 244,
            blue: 12,
            green: 23,
        };

        color *= mul;

        let expected = Color {
            red: 255,
            blue: 144,
            green: 255,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_i8() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: i8 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_u8() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: u8 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_i16() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: i16 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_u16() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: u16 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_i32() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: i32 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_u32() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: u32 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_i64() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: i64 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assign_u64() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: u64 = 12;

        color *= mul;

        let expected = Color {
            red: 144,
            blue: 144,
            green: 144,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assfgn_f32() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: f32 = 12.2;

        color *= mul;

        let expected = Color {
            red: 146,
            blue: 146,
            green: 146,
        };

        assert_eq!(color, expected);
    }

    #[test]
    fn test_mul_assfgn_f64() {
        let mut color = Color {
            red: 12,
            blue: 12,
            green: 12,
        };

        let mul: f64 = 12.2;

        color *= mul;

        let expected = Color {
            red: 146,
            blue: 146,
            green: 146,
        };

        assert_eq!(color, expected);
    }
}
