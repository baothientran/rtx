use crate::core::vec3;
use crate::scene::ray;
use crate::scene::camera;

#[derive(Copy, Clone, Debug)]
pub struct PerspectiveCamera {
    location: vec3::Vec3,
    view_x_axis: vec3::Vec3,
    view_y_axis: vec3::Vec3,
    view_z_axis: vec3::Vec3,
    view_angle: f32,
    distance_to_image: f32,
    image_width: usize,
    image_height: usize,
    image_horizontal_length: f32,
    image_vertical_length: f32,
    top_left_position: vec3::Vec3,
}

impl PerspectiveCamera {
    pub fn new(
        location: vec3::Vec3,
        out_direction: vec3::Vec3,
        up_direction: vec3::Vec3,
        view_angle: f32,
        distance_to_image: f32,
        image_width: usize,
        image_height: usize,
    ) -> PerspectiveCamera {
        let mut view_x_axis = vec3::Vec3::cross(&out_direction, &up_direction);
        view_x_axis = vec3::Vec3::normalize(&view_x_axis).unwrap();

        let mut view_y_axis = vec3::Vec3::cross(&view_x_axis, &out_direction);
        view_y_axis = vec3::Vec3::normalize(&view_y_axis).unwrap();

        let view_z_axis = vec3::Vec3::normalize(&out_direction).unwrap();

        let image_horizontal_length = 2.0 * distance_to_image * f32::tan(view_angle / 2.0);
        let image_vertical_length =
            image_horizontal_length * (image_height as f32) / (image_width as f32);
        let top_left_position = location + distance_to_image * view_z_axis
            - image_horizontal_length / 2.0 * view_x_axis
            + image_vertical_length / 2.0 * view_y_axis;

        PerspectiveCamera {
            location,
            view_x_axis,
            view_y_axis,
            view_z_axis,
            view_angle,
            distance_to_image,
            image_width,
            image_height,
            image_horizontal_length,
            image_vertical_length,
            top_left_position,
        }
    }
}

impl camera::Camera for PerspectiveCamera {
    fn create_ray(&self, x: f32, y: f32) -> ray::Ray {
        let origin = self.location;
        let width_ratio = x / ((self.image_width - 1) as f32);
        let height_ratio = y / ((self.image_height - 1) as f32);
        let mut direction = self.top_left_position
            + self.image_horizontal_length * width_ratio * self.view_x_axis
            - self.image_vertical_length * height_ratio * self.view_y_axis
            - origin;

        direction = vec3::Vec3::normalize(&direction).unwrap();

        return ray::Ray::new(origin, direction);
    }
}

#[cfg(test)]
mod test {
    use super::*;
}