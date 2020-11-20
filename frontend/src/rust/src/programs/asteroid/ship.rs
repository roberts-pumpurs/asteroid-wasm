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
        // self.obj.position.x().rem_euclid(rhs)

        /* Apply drag */
        self.obj.speed = self.obj.speed * 0.9;
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

impl Asteroid {
    pub fn new(gl: &GL, offset_z: f32) -> Self {
        let gl_buffer = Self::init_buffers(gl);
        let buffers = Drawable::new(gl_buffer.0, gl_buffer.1, gl_buffer.2);
        let g_object = GameObject::new(buffers, offset_z);
        Self { obj: g_object }
    }

    fn init_buffers(gl: &GL) -> (i32, i32, WebGlBuffer) {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

        // Construct asteroid
        let mut rng = rand::thread_rng();
        let rand_x = rng.gen_range(-300., 300.);
        let rand_y = rng.gen_range(-300., 300.);

        let radius = 1. + rng.gen_range(0., 1.);

        let mut points: Vec<f32> = vec![];
        let t = 0;
        let x_first = (radius + (t as f32).cos()) * rng.gen_range(0.5, 1.5);
        let y_first = (radius + (t as f32).sin()) * rng.gen_range(0.5, 1.5);
        points.push(x_first);
        points.push(y_first);
        points.push(0.);
        for t in (30..360).step_by(30) {
            let x = (radius + (t as f32).cos()) * rng.gen_range(0.5, 1.5);
            let y = (radius + (t as f32).sin()) * rng.gen_range(0.5, 1.5);
            points.push(x);
            points.push(y);
            points.push(0.);

            points.push(x);
            points.push(y);
            points.push(0.);
        }
        points.push(x_first);
        points.push(y_first);
        points.push(0.);
        let first = points.get(0).unwrap().clone();
        points.push(first);

        unsafe {
            let vert_array = js_sys::Float32Array::view(&points);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        (3, (points.len() / 3) as i32, position_buffer)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.obj.update(delta_time);
    }
}
