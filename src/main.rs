use rtx::core::{image, mat4, math, vec3};
use rtx::exporter::ppm;
use rtx::scene::camera::perspective_camera;
use rtx::scene::light;
use rtx::scene::material::{matte};
use rtx::scene::sampler::random_sampler;
use rtx::scene::shape;
use rtx::scene::world;
use rtx::tracer;
use std::rc;

fn main() {
    // setup image
    let mut img = image::Image::new(1920, 1080);

    // setup scene
    let rectangle_light = Box::new(light::rectangle_light::RectangleLight::new(
        mat4::Mat4::new().translate(&vec3::Vec3::new(0.0, 0.01, 0.5)).rotate(math::degree_to_radian(90.0), &vec3::Vec3::new(1.0, 0.0, 0.0)),
        0.2,
        0.2,
        vec3::Vec3::from(255.0),
    ));
    let plane = rc::Rc::new(shape::plane::Plane::new(
        mat4::Mat4::new(),
        vec3::Vec3::new(0.0, 1.0, 0.0),
        0.0,
    ));
    let sphere_center = rc::Rc::new(shape::sphere::Sphere::new(
        mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.0, 0.2, 0.0)),
        0.2,
    ));
    let white_matte = rc::Rc::new(matte::Matte::new(vec3::Vec3::from(1.0), 0.3));

    let mut world = world::World::new();
    world.add_shape(plane, white_matte.clone());
    world.add_shape(sphere_center, white_matte.clone());
    world.add_light(rectangle_light);

    // setup camera
    let view_location = vec3::Vec3::new(0.0, 1.4, 2.5);
    let mut view_out = vec3::Vec3::from(0.0) - view_location;
    view_out = vec3::Vec3::normalize(&view_out).unwrap();
    let view_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let view_angle = math::degree_to_radian(60.0);
    let distance_to_image = 1.0;
    let camera = perspective_camera::PerspectiveCamera::new(
        view_location,
        view_out,
        view_up,
        view_angle,
        distance_to_image,
        img.width(),
        img.height(),
    );

    // setup sampler
    let mut sampler = random_sampler::RandomSampler::new();

    // render objects
    tracer::monte_carlo::render(&camera, &world, &mut sampler, 10, &mut img);

    // export to file
    ppm::write_to_file("test.ppm", &img).unwrap();
}
