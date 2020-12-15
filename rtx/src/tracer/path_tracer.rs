use crate::core::image;
use crate::core::math;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::ray;
use crate::scene::world;

fn path_trace(ray: &ray::Ray, world: &world::World, _depth: u32) -> Option<vec3::Vec3> {
    if let Some(surface) = world.intersect_ray(ray) {
        let mut lo = vec3::Vec3::from(0.0);
        let normal = surface.normal();
        let surface_material = surface.material();
        let surface_point = *surface.position() + math::EPSILON_F32_4 * *normal;
        let wo = ray.direction();

        // add color from lights around the world
        for light in world.lights() {
            if light.is_visible(&surface_point, world) {
                let mut wi: vec3::Vec3 = vec3::Vec3::from(0.0);
                let li = light.li(&surface_point, &mut wi);
                let brdf = surface_material.brdf(&surface_point, wo, &wi);
                lo += brdf * li * f32::abs(vec3::Vec3::dot(normal, &wi));
            }
        }

        // reflection

        // refraction

        return Some(lo);
    }

    return None;
}

pub fn render(camera: &impl camera::Camera, world: &world::World, image: &mut image::Image) {
    let image_width = image.width();
    let image_height = image.height();

    for y in 0..image_height {
        for x in 0..image_width {
            let ray = camera.create_ray(x as f32, y as f32);
            if let Some(color) = path_trace(&ray, world, 0) {
                image[y][x] = color;
            }
        }
    }
}
