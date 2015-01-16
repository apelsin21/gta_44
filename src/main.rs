#![feature(plugin)]

extern crate cgmath;
extern crate gfx;
extern crate image;

#[macro_use]
#[plugin]
extern crate gfx_macros;
extern crate glfw;

use cgmath::FixedArray;
use cgmath::{Matrix, Point3, Vector3};
use cgmath::{Transform, AffineMatrix3};
use gfx::{Device, DeviceHelper, ToSlice};
use gfx::batch;
use glfw::Context;

mod texture;

#[vertex_format]
#[derive(Copy)]
struct Vertex {
    #[as_float]
    #[name = "v_pos"]
    pos: [i8; 3],

    #[as_float]
    #[name = "v_uv"]
    tex_coord: [u8; 2],
}

#[shader_param(QuadBatch)]
struct Params {
    #[name = "v_transform"]
    transform: [[f32; 4]; 4],

    #[name = "sampler"]
    sampler: gfx::shade::TextureParam,
}

static VERTEX_SRC: gfx::ShaderSource<'static> = shaders! {
glsl_150: b"
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

static FRAGMENT_SRC: gfx::ShaderSource<'static> = shaders! {
glsl_150: b"
    #version 150 core

    in vec2 f_uv;
    out vec4 out_color;

    uniform sampler2D sampler;

    void main() {
        out_color = texture(sampler, f_uv);
    }
"
};

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));

    let (window, events) = glfw
        .create_window(800, 600, "GTA 44", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_char_polling(true);

    let (w, h) = window.get_framebuffer_size();
    let mut frame = gfx::Frame::new(w as u16, h as u16);

    let mut device = gfx::GlDevice::new(|s| window.get_proc_address(s));
    let mut renderer = device.create_renderer();
    let mut context = batch::Context::new();

    let vertex_data = [
        Vertex { pos: [-1, -1,  0], tex_coord: [0, 0] }, //Bottom left
        Vertex { pos: [ 1, -1,  0], tex_coord: [1, 0] }, //Bottom right
        Vertex { pos: [ 1,  1,  0], tex_coord: [1, 1] }, //Top right
        Vertex { pos: [-1, -1,  0], tex_coord: [0, 0] }, //Bottom left
        Vertex { pos: [ 1,  1,  0], tex_coord: [1, 1] }, //Top right
        Vertex { pos: [-1,  1,  0], tex_coord: [0, 1] }, //Top left
    ];

    let mesh = device.create_mesh(&vertex_data);
    
    let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
    
    let texture_info = gfx::tex::TextureInfo {
        width: 2,
        height: 2,
        depth: 1,
        levels: 1,
        kind: gfx::tex::TextureKind::Texture2D,
        format: gfx::tex::RGBA8,
    };
    let image_info = texture_info.to_image_info();
    let texture = device.create_texture(texture_info).unwrap();
  
    let mut test = texture::Texture::new();
    test.load(&Path::new("test.png"));

    let mut texture_data = test.pixels();

    device.update_texture(&texture, &image_info, texture_data.as_slice()).unwrap();

    let sampler = device.create_sampler(
        gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Scale,
                                   gfx::tex::WrapMode::Clamp)
    );

    let program = device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone())
                        .unwrap();
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);

    let batch: QuadBatch = context.make_batch(&program, &mesh, slice, &state).unwrap();

    let view: AffineMatrix3<f32> = Transform::look_at(
        &Point3::new(1.5f32, -5.0, 3.0),
        &Point3::new(0f32, 0.0, 0.0),
        &Vector3::unit_z(),
    );
    let aspect = w as f32 / h as f32;
    let proj = cgmath::perspective(cgmath::deg(60.0f32), aspect, 1.0, 10.0);

    let data = Params {
        transform: proj.mul_m(&view.mat).into_fixed(),
        sampler: (texture, Some(sampler)),
    };

    let clear_data = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0,
    };

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    window.set_should_close(true),
                glfw::WindowEvent::FramebufferSize(w, h) => {
                    frame.width = w as u16;
                    frame.height = h as u16;
                },
                glfw::WindowEvent::Char(c) => {
                    println!("pressed char key {}", c);
                },
                _ => {},
            }
        }

        renderer.clear(clear_data, gfx::COLOR | gfx::DEPTH, &frame);
        renderer.draw(&(&batch, &data, &context), &frame);
        device.submit(renderer.as_buffer());
        renderer.reset();

        window.swap_buffers();
    }
}
