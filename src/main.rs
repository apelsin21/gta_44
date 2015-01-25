#![feature(plugin)]

extern crate cgmath;
extern crate gfx;
extern crate image;

#[macro_use]
#[plugin]
extern crate gfx_macros;
extern crate glfw;

use gfx::{Device, DeviceHelper, ToSlice};
use gfx::batch;
use glfw::Context;

use cgmath::FixedArray;

mod texture;
mod sprite;
mod defs;
mod camera;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(800, 600, "GTA 44", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_char_polling(true);

    let (w, h) = window.get_framebuffer_size();
    let mut frame = gfx::Frame::new(w as u16, h as u16);

    let mut device = gfx::GlDevice::new(|s| window.get_proc_address(s));
    let mut renderer = device.create_renderer();
    let mut context = batch::Context::new();

    let mut sprite = sprite::Sprite::new();
    sprite.load_texture("data/textures/bajs.png");

    let mesh = device.create_mesh(&*sprite.vertices);
    let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
    
    let texture = device.create_texture(sprite.texture.tex_info).unwrap();
    device.update_texture(&texture, &sprite.texture.img_info, &*sprite.texture.pixels).unwrap();
    
    let sampler = device.create_sampler(
        gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Scale,
                                   gfx::tex::WrapMode::Clamp)
    );

    let program = device.link_program(defs::VERTEX_SRC.clone(), defs::FRAGMENT_SRC.clone()).unwrap();
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);

    let batch: defs::SpriteBatch = context.make_batch(&program, &mesh, slice, &state).unwrap();
  
    let ident = cgmath::Matrix4::<f32>::identity().into_fixed();

    let mut shader_data = defs::Params {
        transform: ident, 
        sampler: (texture, Some(sampler)),
    };

    let clear_data = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0,
    };

    let mut camera = camera::Camera::new();
    camera.eye_pos = cgmath::Point3::new(2.0f32, 1.0f32, 4.0f32);

    while !window.should_close() {
        glfw.poll_events();
        
        for(_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    window.set_should_close(true),

                glfw::WindowEvent::FramebufferSize(w, h) => {
                    frame.width = w as u16;
                    frame.height = h as u16;
        
                    camera.aspect = frame.width as f32 / frame.height as f32;
                    println!("aspect updated: {}", camera.aspect);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    println!("scroll event: {}x{}", x, y);
                },
                glfw::WindowEvent::Char(c) => {
                    println!("pressed char key {}", c);
                },
                _ => {},
            }
        }

        let time = glfw.get_time() as f32;
      
        camera.pos = {
            let x = cgmath::sin(cgmath::rad(time));
            let y = cgmath::cos(cgmath::rad(time));
        
            cgmath::Point3::new(x, y, 0.0)
        };

        //Rotate camera around origin
        camera.eye_pos = {
            let x = cgmath::sin(cgmath::rad(0.5*time)) * 5.0;
            let y = cgmath::cos(cgmath::rad(0.5*time)) * 5.0;

            cgmath::Point3::new(x, y, 5.0)
        };

        camera.update();
        shader_data.transform = *camera.get_mvp().as_fixed();

        renderer.clear(clear_data, gfx::COLOR | gfx::DEPTH, &frame);
        renderer.draw(&(&batch, &shader_data, &context), &frame);
        device.submit(renderer.as_buffer());
        renderer.reset();
        

        window.swap_buffers();
    }
}
