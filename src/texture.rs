extern crate image;

pub struct Texture {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            pixels: vec![0u8],
            width: 0u32,
            height: 0u32,
        }
    }

    pub fn load(self: &mut Texture, path: &Path) {
        let image = match image::open(path) {
            Ok(i) => i,
            Err(err) => {
                panic!("failed to load image {}", path.display());
            },
        };

        self.pixels = image.raw_pixels();
    }

    pub fn pixels(self: &Texture) -> &Vec<u8> {
        &self.pixels
    }
}
