extern crate gfx;
extern crate cgmath;

use cgmath::{Point, Matrix};

pub struct Camera {
    pub model: cgmath::Matrix4<f32>,
    pub view: cgmath::Matrix4<f32>,
    pub proj: cgmath::Matrix4<f32>,

    pub pos: cgmath::Point3<f32>,
    pub eye_pos: cgmath::Point3<f32>,
    pub look_pos: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,

    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            model: cgmath::Matrix4::identity(),
            view: cgmath::Matrix4::identity(),
            proj: cgmath::Matrix4::identity(),
        
            pos: cgmath::Point3::new(0.0f32, 0.0, 0.0),
            eye_pos: cgmath::Point3::new(0.0f32, 0.0, 0.0),
            look_pos: cgmath::Point3::new(0.0f32, 0.0, 0.0),
            up: cgmath::Vector3::unit_z(),
        
            fov: 3.1415f32,
            aspect: 1.33f32,
            near: 0.1f32,
            far: 100.0f32,
        }
    }

    pub fn update(self: &mut Camera) {
        self.model = cgmath::Matrix4::from_translation(&self.pos.to_vec());
        
        self.view = cgmath::Matrix4::look_at(
            &self.eye_pos,
            &self.look_pos,
            &self.up);
        
        self.proj = cgmath::perspective(cgmath::rad(self.fov), self.aspect,
                                        self.near, self.far);
    }

    pub fn get_mvp(self: &Camera) -> cgmath::Matrix4<f32> {
        self.proj.mul_m(&self.view)
    }
}
