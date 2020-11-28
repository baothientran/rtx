use crate::core::image;
use crate::core::color;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn write_to_file(file: &str, image: &image::Image) -> std::io::Result<()> {
    let mut writer = match File::create(file) {
        Ok(f) => BufWriter::new(f),
        Err(e) => panic!("Problem open file {}.\nEncounter error: {:#?}", file, e),
    };

    write_to_writer(&mut writer, image)?;
    writer.flush()
}

pub fn write_to_writer(
    writer: &mut impl std::io::Write,
    image: &image::Image,
) -> std::io::Result<()> {
    writer.write_fmt(format_args!(
        "P3\n{width} {height}\n{max_color}\n",
        width = image.width(),
        height = image.height(),
        max_color = 255
    ))?;

    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = color::Color::from_vec3(&image[y][x]);
            writer.write_fmt(format_args!(
                "{} {} {}\n",
                color.red, color.blue, color.green
            ))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufRead;
    use std::io::BufReader;

    // Mock Writer with String buffer
    struct MockStringWriter {
        buffer: String,
    }

    impl MockStringWriter {
        fn new() -> MockStringWriter {
            let mut buffer = String::new();
            buffer.reserve(2000);
            MockStringWriter { buffer }
        }
    }

    impl std::io::Write for MockStringWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            return match std::str::from_utf8(buf) {
                Ok(s) => {
                    self.buffer.push_str(s);
                    Ok(s.len())
                }
                Err(e) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                }
            };
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    // Mock Writer with an error
    struct MockErrorWriter {}

    impl std::io::Write for MockErrorWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Mock Error Write",
            ));
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_write_to_writer() {
        let mut str_writer = MockStringWriter::new();
        let image = image::Image::new(2, 2);
        write_to_writer(&mut str_writer, &image).unwrap();

        // check that the buffer has the write format for ppm
        let buffer = &str_writer.buffer;
        let mut lines = buffer.lines();
        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("2 2"));
        assert_eq!(lines.next(), Some("255"));
        assert_eq!(lines.next(), Some("0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0"));
        assert_eq!(lines.next(), None);
    }

    #[test]
    #[should_panic(expected = "Mock Error Write")]
    fn test_write_to_writer_with_err() {
        let mut error_writer = MockErrorWriter {};
        let image = image::Image::new(2, 2);
        match write_to_writer(&mut error_writer, &image) {
            Ok(_) => {}
            Err(e) => {
                panic!(e.to_string());
            }
        }
    }

    #[test]
    fn test_write_to_file() {
        let image = image::Image::new(2, 2);
        write_to_file("test.ppm", &image).unwrap();
        let mut reader = BufReader::new(std::fs::File::open("test.ppm").unwrap());

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "P3\n");

        line.clear();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "2 2\n");

        line.clear();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "255\n");

        line.clear();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "0 0 0\n");

        line.clear();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "0 0 0\n");

        line.clear();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "0 0 0\n");

        line.clear();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "0 0 0\n");

        std::fs::remove_file("test.ppm").unwrap();
    }
}
