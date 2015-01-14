extern crate image;

pub struct Texture {
    pixels: Vec<u8>,
    width: u16,
    height: u16,
}

pub impl Texture {
    fn new() -> Texture {
        Texture {
            pixels: vec![0u8],
            width: 0u16,
            height: 0u16,
        }
    }

    fn load(self: &mut Texture, path: &Path) {
        let image = match image::open(path) {
            Ok(i) => i,
            Err(_) => {
                panic!("failed to load image");
            },
        };

        self.pixels = image.raw_pixels();

    }
}

pub fn load(path: &Path) -> &[u8] {
}
