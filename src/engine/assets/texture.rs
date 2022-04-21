use std::fs::File;
use std::io::BufReader;


pub struct Texture {
    image_source: image::DynamicImage,
}

impl Texture {
}

impl Texture {
    pub fn build(file: File) -> Self {

        let buf_reader = BufReader::new(file);
        let image_source = image::load(buf_reader, image::ImageFormat::Png).unwrap();

        Self {
            image_source
        }
    }
}