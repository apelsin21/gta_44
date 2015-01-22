extern crate gfx;
extern crate gfx_macros;

use defs;
use texture;

pub struct Sprite {
    pub vertices: Vec<defs::Vertex>,
    pub texture: texture::Texture,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            //A standard quad using six vertices and uvs
            vertices:
                vec![
                    defs::Vertex { pos: [-1, -1, 0], tex_coord: [0, 0] }, //Bottom left
                    defs::Vertex { pos: [ 1, -1, 0], tex_coord: [1, 0] }, //Bottom right
                    defs::Vertex { pos: [ 1,  1, 0], tex_coord: [1, 1] }, //Top right
                    defs::Vertex { pos: [-1, -1, 0], tex_coord: [0, 0] }, //Bottom left
                    defs::Vertex { pos: [ 1,  1, 0], tex_coord: [1, 1] }, //Top right
                    defs::Vertex { pos: [-1,  1, 0], tex_coord: [0, 1] }, //Top left
                ],

            texture: texture::Texture::new(),
        }
    }
    
    pub fn load_texture(self: &mut Sprite, path: &str) {
        self.texture.load(&Path::new(path));
    }
}
