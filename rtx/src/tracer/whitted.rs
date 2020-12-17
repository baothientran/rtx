use crate::core::image;
use crate::core::math;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::material;
use crate::scene::ray;
use crate::scene::world;

fn ray_trace(
    ray: &ray::Ray,
    world: &world::World,
    depth: u32,
    max_depth: u32,
) -> Option<vec3::Vec3> {
    if let Some(surface) = world.intersect_ray(ray) {
        let mut lo = vec3::Vec3::from(0.0);
        let normal = surface.normal();
        let surface_material = surface.material();
        let wo = -*ray.direction();
        let surface_point;
        let dot_normal_wo = vec3::Vec3::dot(normal, &wo);
        if dot_normal_wo > 0.0 {
            surface_point = *surface.position() + math::EPSILON_F32_4 * *normal;
        } else {
            surface_point = *surface.position() - math::EPSILON_F32_4 * *normal;
        }

        // add color from lights around the world
        for light in world.lights() {
            if light.is_visible(&surface_point, world) {
                let mut wi = vec3::Vec3::from(0.0);
                let li = light.li(&surface_point, &mut wi);
                let brdf = surface_material.brdf(dot_normal_wo, normal, &wo, &wi);
                lo += brdf * li * f32::abs(vec3::Vec3::dot(normal, &wi));
            }
        }

        // add reflection or refraction
        if depth <= max_depth {
            if surface_material.has_types(material::MaterialType::Reflection as u32)
                || surface_material.has_types(material::MaterialType::Refraction as u32)
            {
                let mut wi = vec3::Vec3::from(0.0);
                let brdf = surface_material.sample_brdf(dot_normal_wo, normal, &wo, &mut wi);
                let ray = ray::Ray::new(surface_point, wi);
                if let Some(li) = ray_trace(&ray, world, depth + 1, max_depth) {
                    lo += brdf * li * f32::abs(vec3::Vec3::dot(normal, &wi));
                }
            }
        }

        return Some(lo);
    }

    return None;
}

pub fn render(
    camera: &impl camera::Camera,
    world: &world::World,
    max_depth: u32,
    image: &mut image::Image,
) {
    let image_width = image.width();
    let image_height = image.height();

    for y in 0..image_height {
        for x in 0..image_width {
            let ray = camera.create_ray(x as f32, y as f32);
            if let Some(color) = ray_trace(&ray, world, 0, max_depth) {
                image[y][x] = color;
            }
        }
    }
}
