extern crate gfx;
extern crate gfx_macros;

#[vertex_format]
#[derive(Copy)]
pub struct Vertex {
    #[as_float]
    #[name = "v_pos"]
    pub pos: [i8; 3],

    #[as_float]
    #[name = "v_uv"]
    pub tex_coord: [u8; 2],
}

#[shader_param(SpriteBatch)]
pub struct Params {
    #[name = "v_transform"]
    pub transform: [[f32; 4]; 4],

    #[name = "sampler"]
    pub sampler: gfx::shade::TextureParam,
}
pub static VERTEX_SRC: gfx::ShaderSource<'static> = shaders! { glsl_150: b"
    #version 150 core
    
    in vec3 v_pos; 
    in vec2 v_uv;
    
    out vec2 f_uv;
    
    uniform mat4 v_transform;
    
    void main() {
        f_uv = v_uv;
        gl_Position = v_transform * vec4(v_pos, 1.0);
    }
"};

pub static FRAGMENT_SRC: gfx::ShaderSource<'static> = shaders! { glsl_150: b"
	#version 150 core

	in vec2 f_uv;
	out vec4 out_color;

	uniform sampler2D sampler;

	void main() {
		out_color = texture(sampler, f_uv);
	}
"};
