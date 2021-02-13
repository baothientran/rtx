use crate::core::image;
use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;
use crate::scene::camera;
use crate::scene::light;
use crate::scene::material;
use crate::scene::ray;
use crate::scene::sampler;
use crate::scene::world;

fn estimate_one_light(
    light: &dyn light::Light,
    surface_material: &dyn material::Material,
    surface_point_ref: &vec3::Vec3,
    surface_normal_ref: &vec3::Vec3,
    dpdu: &vec3::Vec3,
    wo: &vec3::Vec3,
    world: &world::World,
    sample: &vec2::Vec2,
) -> vec3::Vec3 {
    let mut wi = None;
    let li = light.sample_li(
        &sample,
        world,
        surface_point_ref,
        surface_normal_ref,
        &mut wi,
    );

    if !li.is_none() {
        let bxdf = surface_material.bxdf(&surface_normal_ref, &dpdu, &wo, &wi.unwrap());
        return bxdf * li.unwrap() * f32::abs(vec3::Vec3::dot(&surface_normal_ref, &wi.unwrap()));
    }

    return vec3::Vec3::from(0.0);
}

fn estimate_all_lights(
    surface_material: &dyn material::Material,
    surface_point_ref: &vec3::Vec3,
    surface_normal_ref: &vec3::Vec3,
    dpdu: &vec3::Vec3,
    wo: &vec3::Vec3,
    world: &world::World,
    sampler: &mut dyn sampler::Sampler,
) -> vec3::Vec3 {
    let mut lo = vec3::Vec3::from(0.0);

    for light in world.lights().iter() {
        let mut light_lo = vec3::Vec3::from(0.0);
        for _ in 0..light.num_samples() {
            let sample = sampler.get_2d();
            light_lo += estimate_one_light(
                light.as_ref(),
                surface_material,
                surface_point_ref,
                surface_normal_ref,
                dpdu,
                wo,
                world,
                &sample,
            );
        }

        light_lo /= light.num_samples() as f32;

        lo += light_lo;
    }

    return lo;
}

fn estimate_all_lights_with_uniform_contributions(
    surface_material: &dyn material::Material,
    surface_point_ref: &vec3::Vec3,
    surface_normal_ref: &vec3::Vec3,
    dpdu: &vec3::Vec3,
    wo: &vec3::Vec3,
    world: &world::World,
    sampler: &mut dyn sampler::Sampler,
) -> vec3::Vec3 {
    let light_id = usize::min(
        (sampler.get_1d() * world.lights().len() as f32) as usize,
        world.lights().len() - 1,
    );

    let sample = sampler.get_2d();
    let light = &world.lights()[light_id];
    let light_lo = estimate_one_light(light.as_ref(), surface_material, surface_point_ref, surface_normal_ref, dpdu, wo, world, &sample);
    return light_lo * (world.lights().len() as f32); 
}

fn estimate_all_lights_with_linear_contributions(
    surface_material: &dyn material::Material,
    surface_point_ref: &vec3::Vec3,
    surface_normal_ref: &vec3::Vec3,
    dpdu: &vec3::Vec3,
    wo: &vec3::Vec3,
    world: &world::World,
    sampler: &mut dyn sampler::Sampler,
) {
}

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
        lo += estimate_all_lights(
            surface_material,
            &surface_point_above,
            &normal,
            &dpdu,
            &wo,
            world,
            sampler,
        );

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
