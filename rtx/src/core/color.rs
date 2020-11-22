use std::cmp::{max, min};
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

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

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        let red = self.red as i16 + rhs.red as i16;
        let red = min(red, 255) as u8;

        let blue = self.blue as i16 + rhs.blue as i16;
        let blue = min(blue, 255) as u8;

        let green = self.green as i16 + rhs.green as i16;
        let green = min(green, 255) as u8;

        *self = Self { red, blue, green }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        let red = self.red as i16 - rhs.red as i16;
        let red = max(red, 0) as u8;

        let blue = self.blue as i16 - rhs.blue as i16;
        let blue = max(blue, 0) as u8;

        let green = self.green as i16 - rhs.green as i16;
        let green = max(green, 0) as u8;

        *self = Self { red, blue, green }
    }
}

impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Color) {
        let red = self.red as i16 * rhs.red as i16;
        let red = min(red, 255) as u8;

        let blue = self.blue as i16 * rhs.blue as i16;
        let blue = min(blue, 255) as u8;

        let green = self.green as i16 * rhs.green as i16;
        let green = min(green, 255) as u8;

        *self = Self { red, blue, green }
    }
}

impl ops::MulAssign<i8> for Color {
    fn mul_assign(&mut self, rhs: i8) {
        let red = self.red as i16 * rhs as i16;
        let red = max(min(red, 255), 0) as u8;

        let blue = self.blue as i16 * rhs as i16;
        let blue = max(min(blue, 255), 0) as u8;

        let green = self.green as i16 * rhs as i16;
        let green = max(min(green, 255), 0) as u8;

        *self = Self { red, blue, green }
    }
}

impl ops::MulAssign<u8> for Color {
    fn mul_assign(&mut self, rhs: u8) {
        let red = self.red as u16 * rhs as u16;
        let red = min(red, 255) as u8;

        let blue = self.blue as u16 * rhs as u16;
        let blue = min(blue, 255) as u8;

        let green = self.green as u16 * rhs as u16;
        let green = min(green, 255) as u8;

        *self = Self { red, blue, green }
    }
}

impl ops::MulAssign<i16> for Color {
    fn mul_assign(&mut self, rhs: i16) {
        let red = self.red as i16 * rhs;
        let red = max(min(red, 255), 0) as u8;

        let blue = self.blue as i16 * rhs;
        let blue = max(min(blue, 255), 0) as u8;

        let green = self.green as i16 * rhs;
        let green = max(min(green, 255), 0) as u8;

        *self = Self { red, blue, green };
    }
}

impl ops::MulAssign<u16> for Color {
    fn mul_assign(&mut self, rhs: u16) {
        let red = self.red as u16 * rhs;
        let red = min(red, 255) as u8;

        let blue = self.blue as u16 * rhs;
        let blue = min(blue, 255) as u8;

        let green = self.green as u16 * rhs;
        let green = min(green, 255) as u8;

        *self = Self { red, blue, green };
    }
}

impl ops::MulAssign<i32> for Color {
    fn mul_assign(&mut self, rhs: i32) {
        let red = self.red as i32 * rhs;
        let red = max(min(red, 255), 0) as u8;

        let blue = self.blue as i32 * rhs;
        let blue = max(min(blue, 255), 0) as u8;

        let green = self.green as i32 * rhs;
        let green = max(min(green, 255), 0) as u8;

        *self = Self { red, blue, green };
    }
}

impl ops::MulAssign<u32> for Color {
    fn mul_assign(&mut self, rhs: u32) {
        let red = self.red as u32 * rhs;
        let red = min(red, 255) as u8;

        let blue = self.blue as u32 * rhs;
        let blue = min(blue, 255) as u8;

        let green = self.green as u32 * rhs;
        let green = min(green, 255) as u8;

        *self = Self { red, blue, green }
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        let red = self.red as f32 * rhs;
        let red = f32::max(f32::min(red, 255.0), 0.0) as u8;

        let blue = self.blue as f32 * rhs;
        let blue = f32::max(f32::min(blue, 255.0), 0.0) as u8;

        let green = self.green as f32 * rhs;
        let green = f32::max(f32::min(green, 255.0), 0.0) as u8;

        *self = Self { red, blue, green }
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        let red = self.red as f64 * rhs;
        let red = f64::max(f64::min(red, 255.0), 0.0) as u8;

        let blue = self.blue as f64 * rhs;
        let blue = f64::max(f64::min(blue, 255.0), 0.0) as u8;

        let green = self.green as f64 * rhs;
        let green = f64::max(f64::min(green, 255.0), 0.0) as u8;

        *self = Self { red, blue, green }
    }
}
