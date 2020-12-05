use crate::core::image;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::renderable;
use crate::scene::shape;
use std::vec;

pub fn render(
    camera: &impl camera::Camera,
    renderables: &vec::Vec<renderable::Renderable>,
    image: &mut image::Image,
) {
    let image_width = image.width();
    let image_height = image.height();

    for y in 0..image_height {
        for x in 0..image_width {
            let ray = camera.create_ray(x as f32, y as f32);
            let mut intersect: Option<shape::HitRecord> = None;
            for renderable in renderables.iter() {
                let shape = renderable.shape();
                match intersect.as_mut() {
                    Some(hit_record) => {
                        if let Some(renderable_hit_record) = shape.intersect_ray(&ray) {
                            if hit_record.ray_time > renderable_hit_record.ray_time {
                                *hit_record = renderable_hit_record;
                            }
                        }
                    }
                    None => intersect = shape.intersect_ray(&ray),
                }
            }

            if let Some(hit_record) = intersect {
                // Shade the hit point
                // Use normal for now
                let abs_normal = vec3::Vec3::abs(&hit_record.normal);
                image[y][x] = abs_normal;
            }
        }
    }
}
