use crate::programs::asteroid::WebGlBuffer;
use crate::utils::console_log;
use nalgebra_glm as glm;

use crate::programs::asteroid::{Drawable, GameObject};
use web_sys::WebGlRenderingContext as GL;

pub struct SpaceShip(pub GameObject);

impl SpaceShip {
    pub fn new(gl: &GL, offset_z: f32) -> Self {
        let gl_buffer = SpaceShip::init_buffers(gl);
        let buffers = Drawable::new(gl_buffer.0, gl_buffer.1, gl_buffer.2);
        let g_object = GameObject::new(buffers, offset_z);
        Self(g_object)
    }

    fn init_buffers(gl: &GL) -> (i32, i32, WebGlBuffer) {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

        // Construct spaceship
        let vertices: Vec<(f32, f32, f32)> = vec![
            (-1., -1., 0.),
            (0., 1., 0.),
            (0., 1., 0.),
            (1., -1., 0.),
            (1., -1., 0.),
            (0., -0.5, 0.),
            (0., -0.5, 0.),
            (-1., -1., 0.),
        ];

        let mut result_array: Vec<f32> = Vec::new();
        for elem in vertices.iter() {
            result_array.push(elem.0 / 3.);
            result_array.push(elem.1 / 3.);
            result_array.push(elem.2);
        }
        unsafe {
            let vert_array = js_sys::Float32Array::view(&result_array);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        (3, 8, position_buffer)
    }

    pub fn update(&mut self, delta_time: f32) {
        let frame_velocity = self.0.velocity.scale(delta_time / 1000.);
        self.0.position += frame_velocity;
    }
}

pub struct Bullet(pub GameObject);

impl Bullet {
    pub fn new(gl: &GL, offset_z: f32) -> Self {
        let gl_buffer = Bullet::init_buffers(gl);
        let buffers = Drawable::new(gl_buffer.0, gl_buffer.1, gl_buffer.2);
        let g_object = GameObject::new(buffers, offset_z);
        Self(g_object)
    }

    fn init_buffers(gl: &GL) -> (i32, i32, WebGlBuffer) {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

        // Construct spaceship
        let vertices: Vec<(f32, f32, f32)> = vec![
            (0., 0.5, 0.),
            (0., 0., 0.),
        ];

        let mut result_array: Vec<f32> = Vec::new();
        for elem in vertices.iter() {
            result_array.push(elem.0 / 3.);
            result_array.push(elem.1 / 3.);
            result_array.push(elem.2);
        }
        unsafe {
            let vert_array = js_sys::Float32Array::view(&result_array);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        (3, 2, position_buffer)
    }

    pub fn update(&mut self, delta_time: f32) {
        let frame_velocity = self.0.velocity.scale(delta_time / 1000.);
        self.0.position += frame_velocity;
    }
}
