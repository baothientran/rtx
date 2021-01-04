use crate::core::math;
use crate::core::vec3;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

impl Color {
    pub fn new(red: u8, blue: u8, green: u8) -> Color {
        return Color { red, blue, green };
    }

    pub fn from_vec3(v: &vec3::Vec3) -> Color {
        let gamma_correct = v.powf(1.0 / 2.2);
        let red = (math::clamp(gamma_correct.x, 0.0, 1.0) * 255.999) as u8;
        let blue = (math::clamp(gamma_correct.y, 0.0, 1.0) * 255.999) as u8;
        let green = (math::clamp(gamma_correct.z, 0.0, 1.0) * 255.999) as u8;
        return Color { red, blue, green };
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
    fn test_from_vec3() {
        let v = vec3::Vec3::new(2.0, 0.3, 0.4);
        let color = Color::from_vec3(&v);
        assert_eq!(color.red, 255);
        assert_eq!(color.blue, 148);
        assert_eq!(color.green, 168);
    }
}
