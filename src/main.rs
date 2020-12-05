use rtx::apps;
use rtx::core::{image, math, vec3};
use rtx::exporter::ppm;
use rtx::scene::perspective_camera::PerspectiveCamera;
use rtx::scene::renderable::{plane as renderable_plane, sphere as renderable_sphere, Renderable};

fn main() {
    // setup scene
    let mut img = image::Image::new(1000, 500);

    let mut renderables = Vec::<Box<dyn Renderable>>::new();
    renderables.push(Box::new(renderable_plane::RenderablePlane::new(
        vec3::Vec3::new(0.0, 1.0, 0.0),
        0.2,
    )));
    renderables.push(Box::new(renderable_sphere::RenderableSphere::new(
        vec3::Vec3::new(0.0, 0.0, 0.0),
        0.2,
    )));

    let view_location = vec3::Vec3::new(0.0, 0.1, 2.0);
    let mut view_out = vec3::Vec3::from(0.0) - view_location;
    view_out = vec3::Vec3::normalize(&view_out).unwrap();
    let view_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let view_angle = math::degree_to_radian(60.0);
    let distance_to_image = 10.0;
    let camera = PerspectiveCamera::new(
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
    apps::cli::render(&camera, &renderables, &mut img);

    // export to file
    ppm::write_to_file("test.ppm", &img).unwrap();
}
