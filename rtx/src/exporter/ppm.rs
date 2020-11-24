use crate::core::image;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn to_file(file: &str, image: &image::Image) {
    let mut writer = match File::create(file) {
        Ok(f) => BufWriter::new(f),
        Err(e) => panic!("Problem open file {}.\nEncounter error: {:#?}", file, e),
    };

    to_writer(&mut writer, image);
    writer.flush().unwrap();
}

pub fn to_writer(writer: &mut impl std::io::Write, image: &image::Image) {
    writer
        .write_fmt(format_args!(
            "P3\n{width} {height}\n{max_color}\n",
            width = image.width(),
            height = image.height(),
            max_color = 255
        ))
        .unwrap();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = image[y][x];
            writer
                .write_fmt(format_args!(
                    "{} {} {}\n",
                    color.red, color.blue, color.green
                ))
                .unwrap();
        }
    }
}
