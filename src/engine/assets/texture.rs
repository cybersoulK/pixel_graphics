use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;


pub struct Texture {
    image_source: image::DynamicImage,
}

impl Texture {
    pub fn build(file: File) -> Rc<Self> {

        let buf_reader = BufReader::new(file);
        let image_source = image::load(buf_reader, image::ImageFormat::Png).unwrap();

        Rc::new(Self {
            image_source
        })
    }
}