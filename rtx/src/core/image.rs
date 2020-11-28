use crate::core::vec3;
use std::ops;
use std::vec::Vec;

pub struct Image {
    colors: Vec<Vec<vec3::Vec3>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let black = vec3::Vec3::new(0.0, 0.0, 0.0);
        return Image {
            colors: vec![vec!(black; width); height],
        };
    }

    pub fn width(&self) -> usize {
        return self.colors.first().unwrap().len();
    }

    pub fn height(&self) -> usize {
        return self.colors.len();
    }
}

impl ops::Index<usize> for Image {
    type Output = Vec<vec3::Vec3>;

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.colors[idx];
    }
}

impl ops::IndexMut<usize> for Image {
    fn index_mut(&mut self, idx: usize) -> &mut Vec<vec3::Vec3> {
        return &mut self.colors[idx];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_create_image() {
        let image = Image::new(20, 20);
        assert_eq!(image.width(), 20);
        assert_eq!(image.height(), 20);

        for y in 0..image.height() {
            for x in 0..image.width() {
                let result = image[y][x];
                assert!(math::equal_epsilon_f32(result.x, 0.0, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(result.y, 0.0, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(result.z, 0.0, math::EPSILON_F32_5));
            }
        }
    }

    #[test]
    fn test_assign_color() {
        let mut image = Image::new(20, 20);
        image[12][12] = vec3::Vec3::new(12.0, 23.0, 10.0);
        let result = image[12][12];
        assert!(math::equal_epsilon_f32(result.x, 12.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.y, 23.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(result.z, 10.0, math::EPSILON_F32_5));
    }
}
