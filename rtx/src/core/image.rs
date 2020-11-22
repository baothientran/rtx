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
