use super::color;
use std::ops;
use std::vec::Vec;

pub struct Image {
    colors: Vec<Vec<color::Color>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let black = color::Color {
            red: 0,
            blue: 0,
            green: 0,
        };
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
    type Output = Vec<color::Color>;

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.colors[idx];
    }
}

impl ops::IndexMut<usize> for Image {
    fn index_mut(&mut self, idx: usize) -> &mut Vec<color::Color> {
        return &mut self.colors[idx];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_image() {
        let image = Image::new(20, 20);
        assert_eq!(image.width(), 20);
        assert_eq!(image.height(), 20);

        for y in 0..image.height() {
            for x in 0..image.width() {
                assert_eq!(image[y][x], color::Color::new(0, 0, 0));
            }
        }
    }

    #[test]
    fn test_assign_color() {
        let mut image = Image::new(20, 20);
        image[12][12] = color::Color::new(12, 23, 10);
        assert_eq!(image[12][12].red, 12);
        assert_eq!(image[12][12].blue, 23);
        assert_eq!(image[12][12].green, 10);
    }
}
