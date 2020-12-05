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
            let mut curr_surface_info: Option<renderable::SurfaceInfo> = None;
            for renderable in renderables.iter() {
                match curr_surface_info.as_mut() {
                    Some(surface_info) => {
                        if let Some(renderable_surface_info) = renderable.intersect_ray(&ray) {
                            if surface_info.ray_time > renderable_surface_info.ray_time {
                                *surface_info = renderable_surface_info;
                            }
                        }
                    }
                    None => curr_surface_info = renderable.intersect_ray(&ray),
                }
            }

            if let Some(surface_info) = curr_surface_info {
                // Shade the hit point
                // Use normal for now
                let abs_normal = vec3::Vec3::abs(&surface_info.normal);
                image[y][x] = abs_normal;
            }
        }
    }
}
