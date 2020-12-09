use crate::core::image;
use crate::core::mat4;
use crate::core::math;
use crate::core::vec3;
use crate::core::vec4;
use crate::scene::camera;
use crate::scene::ray;
use crate::scene::world;
use rand::Rng;

fn path_trace(ray: &ray::Ray, world: &world::World, depth: u32) -> Option<vec3::Vec3> {
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

        // shoot random rays around the scene to sample global lights
        let num_samples = 16;
        let TBN;
        let mut rng = rand::thread_rng();
        let mut indirect_lo = vec3::Vec3::from(0.0);
        for _i in 0..num_samples {
            let phi: f32 = math::degree_to_radian(rng.gen_range(0.0, 360.0));
            let theta: f32 = math::degree_to_radian(rng.gen_range(0.0, 90.0));
            let sample_position = vec3::Vec3::new(
                f32::sin(theta) * f32::cos(phi),
                f32::cos(theta),
                f32::sin(theta) * f32::sin(phi),
            );

            // construct secondary ray to sample color around the scene
            let mut vec4_sample_position = TBN * vec4::Vec4::from_vec3(&sample_position, 1.0);
            sample_position = vec4::Vec4::to_vec3(&vec4_sample_position);

            let mut wi = sample_position - surface_point;
            wi = vec3::Vec3::normalize(&wi).unwrap();
            let secondary_ray = ray::Ray::new(surface_point, wi);
            if let Some(li) = path_trace(&secondary_ray, world, depth + 1) {
                let brdf = surface_material.brdf(&surface_point, wo, &wi);
                indirect_lo += brdf * li * f32::abs(vec3::Vec3::dot(normal, &wi));
            }
        }
        indirect_lo /= num_samples as f32; // may not be correct.
        lo += indirect_lo;

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
