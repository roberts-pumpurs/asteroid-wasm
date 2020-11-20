use crate::programs::asteroid::get_vec2_from_vec3;
use crate::programs::asteroid::WebGlBuffer;
use crate::utils::console_log;
use nalgebra_glm as glm;

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
        // Update matrix values
        // Update rotation matrix
        self.obj.rotation = get_matrix_rotation(self.obj.angle);

        // Update direction matrix
        let dir3 = self.obj.rotation.mul_vec3(bevy_math::Vec3::new(0., 1., 0.));
        self.obj.direction = get_vec2_from_vec3(&dir3);

        // Update translation matrix
        let velocity = self.obj.direction * self.obj.speed * delta_time;
        self.obj.position += velocity;

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
        let gl_buffer = Bullet::init_buffers(gl);
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
        self.0.rotation = get_matrix_rotation(self.0.angle);

        // Update direction matrix
        let dir3 = self.0.rotation.mul_vec3(bevy_math::Vec3::new(0., 1., 0.));
        self.0.direction = get_vec2_from_vec3(&dir3);

        // Update translation matrix
        let velocity = self.0.direction * self.0.speed * delta_time;
        self.0.position += velocity;
    }
}
