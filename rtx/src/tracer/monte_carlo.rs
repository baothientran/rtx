use crate::core::image;
use crate::core::math;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::ray;
use crate::scene::sampler;
use crate::scene::world;

fn ray_trace(
    ray: &ray::Ray,
    world: &world::World,
    sampler: &mut dyn sampler::Sampler,
    _depth: u32,
    _max_depth: u32,
) -> Option<vec3::Vec3> {
    if let Some(renderable_surface) = world.intersect_ray(ray) {
        let mut lo = vec3::Vec3::from(0.0);
        let surface = renderable_surface.shape_surface();
        let dpdu = surface.calc_world_dpdu();
        let normal = surface.calc_world_normal();
        let surface_material = renderable_surface.material();
        let wo = -ray.direction();
        let surface_point_above;
        let _surface_point_below;
        let dot_normal_wo = vec3::Vec3::dot(&normal, &wo);
        if dot_normal_wo > 0.0 {
            let surface_point = surface.calc_world_position();
            surface_point_above = surface_point + math::EPSILON_F32_4 * normal;
            _surface_point_below = surface_point - math::EPSILON_F32_4 * normal;
        } else {
            let surface_point = surface.calc_world_position();
            surface_point_above = surface_point - math::EPSILON_F32_4 * normal;
            _surface_point_below = surface_point + math::EPSILON_F32_4 * normal;
        }

        // add color from lights around the world
        let mut direct_light_lo = vec3::Vec3::from(0.0);
        for light in world.lights() {
            let mut light_lo = vec3::Vec3::from(0.0);
            for _ in 0..light.num_samples() {
                let mut wi = vec3::Vec3::from(0.0);
                let li = light.sample_li(sampler, world, &surface_point_above, &normal, &mut wi);
                if !vec3::Vec3::equal_epsilon(&li, &vec3::Vec3::from(0.0), math::EPSILON_F32_6) {
                    let bxdf = surface_material.bxdf(&normal, &dpdu, &wo, &wi);
                    light_lo += bxdf * li * f32::abs(vec3::Vec3::dot(&normal, &wi));
                }
            }
            light_lo /= light.num_samples() as f32;
            direct_light_lo += light_lo;
        }

        direct_light_lo /= world.lights().len() as f32;

        lo += direct_light_lo;
        return Some(lo);
    }

    return None;
}

pub fn render(
    camera: &impl camera::Camera,
    world: &world::World,
    sampler: &mut dyn sampler::Sampler,
    max_depth: u32,
    image: &mut image::Image,
) {
    let image_width = image.width();
    let image_height = image.height();

    for y in 0..image_height {
        for x in 0..image_width {
            let ray = camera.create_ray(x as f32, y as f32);
            if let Some(color) = ray_trace(&ray, world, sampler, 0, max_depth) {
                image[y][x] = color;
            }
        }
    }
}
