use crate::core::image;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::renderable;
use std::vec;

pub fn render(
    camera: &impl camera::Camera,
    renderables: &vec::Vec<Box<dyn renderable::Renderable>>,
    image: &mut image::Image,
) {
    let image_width = image.width();
    let image_height = image.height();

    for y in 0..image_height {
        for x in 0..image_width {
            let ray = camera.create_ray(x as f32, y as f32);
            let mut surface_info = renderable::SurfaceInfo::None;
            for renderable in renderables.iter() {
                renderable.intersect_ray(&ray, &mut surface_info);
            }

            match surface_info {
                renderable::SurfaceInfo::Hit {
                    ray_time: _,
                    position: _,
                    normal,
                } => {
                    // Shade the hit point
                    // Use normal for now
                    let abs_normal = vec3::Vec3::abs(&normal);
                    image[y][x] = abs_normal;
                }
                renderable::SurfaceInfo::None => {}
            }
        }
    }
}
