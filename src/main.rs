use rtx::core::{image, mat4, math, vec3};
use rtx::exporter::ppm;
use rtx::scene::camera::perspective_camera;
use rtx::scene::light;
use rtx::scene::material::matte;
use rtx::scene::sampler::random_sampler;
use rtx::scene::shape;
use rtx::scene::world;
use rtx::tracer;
use std::rc;

fn main() {
    // setup image
    let mut img = image::Image::new(1000, 500);

    // setup lights
    let rectangle_light_shape = Box::new(shape::rectangle::Rectangle::new(
        mat4::Mat4::new()
            .translate(&vec3::Vec3::new(0.0, 0.8, 0.5))
            .rotate(
                math::degree_to_radian(90.0),
                &vec3::Vec3::new(1.0, 0.0, 0.0),
            ),
        0.4,
        0.4,
    ));
    let sphere_light_right_shape = Box::new(shape::sphere::Sphere::new(
        mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.4, 0.3, 0.0)),
        0.2,
    ));
    let sphere_light_left_shape = Box::new(shape::sphere::Sphere::new(
        mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(-0.4, 0.3, 0.0)),
        0.2,
    ));
    let sphere_area_light_right = Box::new(light::area_light::AreaLight::new(
        vec3::Vec3::from(1.0),
        sphere_light_right_shape,
        32,
    ));
    let sphere_area_light_left = Box::new(light::area_light::AreaLight::new(
        vec3::Vec3::from(1.0),
        sphere_light_left_shape,
        32,
    ));
    let rectangle_area_light = Box::new(light::area_light::AreaLight::new(
        vec3::Vec3::from(1.0),
        rectangle_light_shape,
        32,
    ));

    // setup shape
    let plane = rc::Rc::new(shape::rectangle::Rectangle::new(
        mat4::Mat4::new()
            .translate(&vec3::Vec3::new(0.0, 0.0, 0.0))
            .rotate(
                math::degree_to_radian(-90.0),
                &vec3::Vec3::new(1.0, 0.0, 0.0).normalize().unwrap(),
            ),
        6.0,
        3.0,
    ));
    let sphere_center = rc::Rc::new(shape::sphere::Sphere::new(
        mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.0, 0.3, 0.0)),
        0.2,
    ));
    let disk = rc::Rc::new(shape::disk::Disk::new(
        mat4::Mat4::translate(&mat4::Mat4::new(), &vec3::Vec3::new(0.0, 0.1, 0.0)).rotate(
            math::degree_to_radian(-90.0),
            &vec3::Vec3::new(1.0, 0.0, 0.0).normalize().unwrap(),
        ),
        0.2,
        0.4,
    ));
    let white_matte = rc::Rc::new(matte::Matte::new(vec3::Vec3::from(0.5), 0.0));

    let mut world = world::World::new();
    world.add_shape(plane, white_matte.clone());
    world.add_shape(sphere_center, white_matte.clone());
    world.add_shape(disk, white_matte.clone());
    world.add_light(sphere_area_light_right);
    world.add_light(sphere_area_light_left);
    world.add_light(rectangle_area_light);

    // setup camera
    let view_location = vec3::Vec3::new(0.0, 1.0, 2.5);
    let mut view_out = vec3::Vec3::from(0.0) - view_location;
    view_out = vec3::Vec3::normalize(&view_out).unwrap();
    let view_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let view_angle = math::degree_to_radian(60.0);
    let distance_to_image = 100.0;
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
