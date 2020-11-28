use crate::core::image;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::hittable;
use std::vec;

pub fn render_perspective(
    camera: &camera::PerspectiveCamera,
    hittables: &vec::Vec<impl hittable::Hittable>,
    image: &mut image::Image,
) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let ray = camera.create_ray(x as f32, y as f32);
            for hittable in hittables.iter() {
                match hittable.hit(&ray) {
                    Some(hitrecord) => {
                        let mut normal = hitrecord.normal;
                        normal = vec3::Vec3::abs(&normal);
                        image[y][x] = normal;
                    }
                    None => {}
                }
            }
        }
    }    
}
