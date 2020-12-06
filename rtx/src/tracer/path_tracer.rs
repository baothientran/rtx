use crate::core::image;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::world;

pub fn render(camera: &impl camera::Camera, world: &world::World, image: &mut image::Image) {
    let image_width = image.width();
    let image_height = image.height();

    for y in 0..image_height {
        for x in 0..image_width {
            let ray = camera.create_ray(x as f32, y as f32);
            if let Some(surface) = world.intersect_ray(&ray) {
                let abs_normal = vec3::Vec3::abs(surface.normal());
                image[y][x] = abs_normal;
            }
        }
    }
}
