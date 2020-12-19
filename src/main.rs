use rtx::core::{image, math, vec3};
use rtx::exporter::ppm;
use rtx::scene::camera::perspective_camera;
use rtx::scene::light;
use rtx::scene::material::{lambertian, reflection, refraction};
use rtx::scene::fresnel::{dielectrics};
use rtx::scene::shape;
use rtx::scene::world;
use rtx::tracer;
use std::rc;

fn main() {
    // setup image
    let mut img = image::Image::new(1000, 500);

    // setup scene
    let plane = rc::Rc::new(shape::plane::Plane::new(
        vec3::Vec3::new(0.0, 1.0, 0.0),
        0.2,
    ));
    let sphere_left = rc::Rc::new(shape::sphere::Sphere::new(
        vec3::Vec3::new(-0.4, 0.0, 0.0),
        0.2,
    ));
    let sphere_behind = rc::Rc::new(shape::sphere::Sphere::new(
        vec3::Vec3::new(0.0, 0.0, -0.4),
        0.2,
    ));
    let sphere_center = rc::Rc::new(shape::sphere::Sphere::new(
        vec3::Vec3::new(0.0, 0.0, 0.0),
        0.2,
    ));
    let sphere_right = rc::Rc::new(shape::sphere::Sphere::new(
        vec3::Vec3::new(0.4, 0.0, 0.0),
        0.2,
    ));
    let mirror = rc::Rc::new(reflection::Reflection::new(
        vec3::Vec3::from(1.0),
        rc::Rc::new(dielectrics::Dielectrics::new(1.0, 1.5)),
    ));
    let glass = rc::Rc::new(refraction::Refraction::new(
        1.0,
        1.05,
        vec3::Vec3::from(1.0),
    ));
    let green_lambertian = rc::Rc::new(lambertian::Lambertian::new(vec3::Vec3::new(0.5, 0.8, 0.7)));
    let purple_lambertian =
        rc::Rc::new(lambertian::Lambertian::new(vec3::Vec3::new(0.8, 0.6, 0.7)));
    let blue_lambertian = rc::Rc::new(lambertian::Lambertian::new(vec3::Vec3::new(0.3, 0.6, 0.7)));

    let point_light_front = Box::new(light::point_light::PointLight::new(
        vec3::Vec3::new(0.0, 2.0, 2.0),
        vec3::Vec3::from(1.0),
        10.0,
    ));
    let point_light_center = Box::new(light::point_light::PointLight::new(
        vec3::Vec3::new(0.0, 0.5, 0.0),
        vec3::Vec3::from(1.0),
        10.0,
    ));
    let point_light_back = Box::new(light::point_light::PointLight::new(
        vec3::Vec3::new(0.0, 2.0, -2.0),
        vec3::Vec3::from(1.0),
        10.0,
    ));

    let mut world = world::World::new();
    world.add_shape(plane, green_lambertian);
    world.add_shape(sphere_left, blue_lambertian);
    world.add_shape(sphere_center, glass);
    world.add_shape(sphere_behind, mirror);
    world.add_shape(sphere_right, purple_lambertian);
    world.add_light(point_light_front);
    world.add_light(point_light_center);
    world.add_light(point_light_back);

    // setup camera
    let view_location = vec3::Vec3::new(-1.5, 1.0, 1.5);
    let mut view_out = vec3::Vec3::from(0.0) - view_location;
    view_out = vec3::Vec3::normalize(&view_out).unwrap();
    let view_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let view_angle = math::degree_to_radian(60.0);
    let distance_to_image = 10.0;
    let camera = perspective_camera::PerspectiveCamera::new(
        view_location,
        view_out,
        view_up,
        view_angle,
        distance_to_image,
        img.width(),
        img.height(),
    );

    // render objects
    tracer::whitted::render(&camera, &world, 10, &mut img);

    // export to file
    ppm::write_to_file("test.ppm", &img).unwrap();
}
