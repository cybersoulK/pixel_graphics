
pub struct Model {}

pub struct Mesh {
    vertices: Vec<glam::Vec3>,
    norms: Vec<glam::Vec3>,

    indexes: Vec<usize>,
}

pub struct Texture {
    image_source: image::DynamicImage,
}

impl Texture {
    fn new(filename: &str) -> Self {

        let file = std::fs::File::open("asd").unwrap();
        let reader = std::io::BufReader::new(file);

        let image_source = image::load(reader, image::ImageFormat::Png).unwrap();

        Self {
            image_source
        }
    }
}