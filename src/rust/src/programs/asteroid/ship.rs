use std::f32::consts::PI;

use crate::programs::asteroid::get_vec2_from_vec3;
use crate::programs::asteroid::WebGlBuffer;
use crate::utils::console_log;
use rand::prelude::*;

use crate::programs::asteroid::{Drawable, GameObject};
use web_sys::WebGlRenderingContext as GL;

use super::get_matrix_rotation;

pub struct SpaceShip {
    pub obj: GameObject,
    pub last_shot: f32,
}

impl SpaceShip {
    pub fn new(gl: &GL, offset_z: f32) -> Self {
        let gl_buffer = SpaceShip::init_buffers(gl);
        let buffers = Drawable::new(gl_buffer.0, gl_buffer.1, gl_buffer.2);
        let g_object = GameObject::new(buffers, offset_z);
        Self {
            obj: g_object,
            last_shot: 0.,
        }
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
        self.last_shot += delta_time;
        self.obj.update(delta_time);

        /* Wrap player */
        if (self.obj.position.y() / 11.).abs() + 0.6 > 1. {
            self.obj.position.set_y(-self.obj.position.y());
        }
        if (self.obj.position.x() / 11.).abs() * 1.6 > 1. {
            self.obj.position.set_x(-self.obj.position.x());
        }

        /* Apply drag */
        self.obj.speed = self.obj.speed * 0.99;
    }
}

pub struct Bullet(pub GameObject);

impl Bullet {
    pub fn new(gl: &GL, offset_z: f32) -> Self {
        let gl_buffer = Self::init_buffers(gl);
        let buffers = Drawable::new(gl_buffer.0, gl_buffer.1, gl_buffer.2);
        let g_object = GameObject::new(buffers, offset_z);
        Self(g_object)
    }

    fn init_buffers(gl: &GL) -> (i32, i32, WebGlBuffer) {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

        // Construct spaceship
        let vertices: Vec<(f32, f32, f32)> = vec![(0., 0.5, 0.), (0., 0., 0.)];

        let mut result_array: Vec<f32> = Vec::new();
        for elem in vertices.iter() {
            result_array.push(elem.0);
            result_array.push(elem.1);
            result_array.push(elem.2);
        }
        unsafe {
            let vert_array = js_sys::Float32Array::view(&result_array);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        (3, 2, position_buffer)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.0.update(delta_time);
    }
}

pub struct Asteroid {
    pub obj: GameObject,
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        // Comparing memory addresses
        self == other
    }
}


// RADIUSES = [4, 2.8, 1.5];
impl Asteroid {

    pub fn new(gl: &GL, offset_z: f32, radius: f32) -> Self {
        let gl_buffer = Self::init_buffers(gl, radius);
        let buffers = Drawable::new(gl_buffer.0, gl_buffer.1, gl_buffer.2);
        let mut g_object = GameObject::new(buffers, offset_z);
        g_object.radius = radius;
        Self { obj: g_object }
    }

    fn init_buffers(gl: &GL, radius: f32) -> (i32, i32, WebGlBuffer) {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

        // Construct asteroid
        let mut rng = rand::thread_rng();
        let mut points: Vec<(f32, f32, f32)> = vec![];
        for i in 0..12 {
            let rotation = (i as f32 / 12.) as f32 * 2. * PI;
            let x = rotation.cos() + rng.gen_range(-0.5, 0.5);
            let y = rotation.sin() + rng.gen_range(-0.5, 0.5);
            let vert_dist = radius; // + rng.gen_range(0.3 * radius, 0.6 * radius);
            // points.push((x * vert_dist, y * vert_dist, 0.));
            points.push((x * vert_dist, y * vert_dist, 0.));
        }

        let mut result_array: Vec<f32> = Vec::new();
        let first =points.get(0).unwrap().clone();
        result_array.push(first.0);
        result_array.push(first.1);
        result_array.push(first.2);
        for elem in points.iter() {
            result_array.push(elem.0);
            result_array.push(elem.1);
            result_array.push(elem.2);
            result_array.push(elem.0);
            result_array.push(elem.1);
            result_array.push(elem.2);
        }
        result_array.push(first.0);
        result_array.push(first.1);
        result_array.push(first.2);
        unsafe {
            let vert_array = js_sys::Float32Array::view(&result_array);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        (3, points.len() as i32 * 2 + 2, position_buffer)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.obj.update(delta_time);
    }
}
