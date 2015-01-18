extern crate image;
extern crate gfx;
extern crate gfx_macros;

pub struct Texture {
    pub pixels: Vec<u8>,

    pub tex_info: gfx::tex::TextureInfo,
    pub img_info: gfx::tex::ImageInfo,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            pixels: vec![0u8],

            tex_info: gfx::tex::TextureInfo::new(),
            img_info: gfx::tex::ImageInfo::new(),
        }
    }

    pub fn load(self: &mut Texture, path: &Path) {
        let image  = match image::open(path) {
            Ok(i) => i,
            Err(_) => {
                panic!("failed to load image {}", path.display());
            },
        };

        let image = image.to_rgba();

        self.tex_info.width = image.width() as u16;
        self.tex_info.height = image.height() as u16;
        
        self.img_info = self.tex_info.to_image_info();
        
        self.pixels = image.into_vec();
    }
}
