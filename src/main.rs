use rtx::core::{image, math, vec3};
use rtx::exporter::ppm;
use rtx::scene::camera::perspective_camera;
use rtx::scene::light;
use rtx::scene::material::lambertian;
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
    let sphere = rc::Rc::new(shape::sphere::Sphere::new(
        vec3::Vec3::new(0.0, 0.0, 0.0),
        0.2,
    ));
    let green_lambertian = rc::Rc::new(lambertian::Lambertian::new(vec3::Vec3::new(0.5, 0.8, 0.7)));
    let purple_lambertian =
        rc::Rc::new(lambertian::Lambertian::new(vec3::Vec3::new(0.8, 0.6, 0.7)));

    let point_light = Box::new(light::point_light::PointLight::new(
        vec3::Vec3::new(0.0, 1.0, 1.0),
        vec3::Vec3::from(2.0),
        6.0,
    ));

    let mut world = world::World::new();
    world.add_shape(plane, green_lambertian);
    world.add_shape(sphere, purple_lambertian);
    world.add_light(point_light);

    // setup camera
    let view_location = vec3::Vec3::new(2.0, 0.3, 2.0);
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

    // render background
    let image_width = img.width();
    let image_height = img.height();
    let start_background = vec3::Vec3::new(0.9, 0.9, 0.9);
    let end_background = vec3::Vec3::new(0.5, 0.7, 1.0);

    for y in 0..image_height {
        for x in 0..image_width {
            // Assign background to image
            let ratio = 1.0 - y as f32 / image_height as f32;
            let background = math::lerp(ratio, start_background, end_background);
            img[y][x] = background;
        }
    }

    // render objects
    tracer::path_tracer::render(&camera, &world, &mut img);

    // export to file
    ppm::write_to_file("test.ppm", &img).unwrap();
}
