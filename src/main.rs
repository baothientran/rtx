use rtx::core::image;
use rtx::exporter::ppm;

fn main() -> std::io::Result<()> {
    let mut image = image::Image::new(500, 500);
    for y in 0..image.height() {
        for x in 0..image.width() {
            image[y][x].red = ((y as f32) / (image.height() as f32) * 255.999) as u8;
            image[y][x].blue = ((x as f32) / (image.width() as f32) * 255.999) as u8;
            image[y][x].green = (0.25 * 255.999) as u8;
        }
    }
    ppm::write_to_file("Test.ppm", &image)
}
