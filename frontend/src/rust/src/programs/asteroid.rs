pub mod shaders;
use crate::canvas::CanvasData;
use crate::input::UserInput;
use crate::transform::Transform;
use crate::utils::console_log;
use crate::RenderObjectTrait;
use core::f32::consts::PI;
use nalgebra_glm as glm;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;

use super::box_2d::UniformLocations;

pub struct AttributeLocationsLocal {
    pub vertex_position: i32,
}

pub struct Drawable {
    pub item_size: i32,
    pub num_items: i32,
    pub buffer_vertices: WebGlBuffer,
}

pub struct SpaceShip {
    pub velocity: glm::TVec3<f32>,
    pub position: glm::TVec3<f32>,
    pub rotation: f32,
    pub buffers: Drawable,
}

const Z_AXIS: f32 = -6.;

pub fn get_matrix_rotation(theta: f32) -> glm::Mat3 {
    let theta_rad = theta * PI / 180.;
    let theta_cos = theta_rad.cos();
    let theta_sin = theta_rad.sin();
    glm::mat3(
        theta_cos, -theta_sin, 0., theta_sin, theta_cos, 0., 0., 0., 1.,
    )
}

impl SpaceShip {
    pub fn draw(
        &self,
        gl: &GL,
        attribute_locations: &AttributeLocationsLocal,
        uniform_locations: &UniformLocations,
        projection_matrix: glm::TMat4<f32>,
    ) {
        console_log("Drawing SpaceShip");

        {
            // Set vertices
            let buffer_type = GL::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffers.buffer_vertices));
            gl.vertex_attrib_pointer_with_i32(
                attribute_locations.vertex_position as u32,
                self.buffers.item_size,
                buffer_type,
                normalize,
                stride,
                offset,
            );
        }

        // set to new position
        let mut empty_matrix = glm::mat4x4(
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        empty_matrix.fill_with_identity();
        /* Perform rotation */
        let rotation_vector = glm::vec3(0., 0., 1.);
        let model_view_matrix =
            glm::rotate_normalized_axis(&empty_matrix, self.rotation, &rotation_vector);
        /* Perform positional movement */
        let translation_vector = self.position;
        let model_view_matrix = glm::translate(&model_view_matrix, &translation_vector);

        gl.uniform_matrix4fv_with_f32_array(
            Some(&uniform_locations.model_view_matrix),
            false,
            model_view_matrix.as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&uniform_locations.projection_matrix),
            false,
            projection_matrix.as_slice(),
        );

        let offset = 0;
        gl.draw_arrays(GL::LINES, offset, self.buffers.num_items);
    }

    fn init_buffers(gl: &GL, vertices: Vec<(f32, f32, f32)>) -> WebGlBuffer {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

        let mut result_array: Vec<f32> = Vec::new();
        for elem in vertices.iter() {
            result_array.push(elem.0 / 3.);
            result_array.push(elem.1 / 3.);
            result_array.push(elem.2);
        }
        console_log(&format!("{:?}", &result_array));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&result_array);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        position_buffer
    }

    fn update(&mut self, delta_time: f32) {
        let frame_velocity = self.velocity.scale(delta_time / 1000.);

        console_log(&format!(
            "OG vel  {:?} frame_velocity {:?}",
            &self.velocity, &frame_velocity
        ));
        self.position += frame_velocity;
    }
}

pub struct AsteroidCanvas {
    // Game itself
    pub ship: SpaceShip,
    pub input: UserInput,
    pub transform: Transform,
    // GL
    program: WebGlProgram,
    attribute_locations: AttributeLocationsLocal,
    uniform_locations: UniformLocations,
}

impl RenderObjectTrait for AsteroidCanvas {
    fn new(gl: &GL, program: WebGlProgram, transform: Transform) -> Self
    where
        Self: Sized,
    {
        // Store metadata
        let input = UserInput::new();

        // Construct spaceship
        let vertices: Vec<(f32, f32, f32)> = vec![
            (-1., -1., 0.),
            (0., 1., 0.),
            (0., 1., 0.),
            (1., -1., 0.),
            (1., -1., 0.),
            (-1., -1., 0.),
        ];
        let vertices = SpaceShip::init_buffers(gl, vertices);
        let mut ship = SpaceShip {
            position: glm::vec3(0., 0., Z_AXIS),
            velocity: glm::vec3(0., 0., 0.),
            rotation: 0.,
            buffers: Drawable {
                item_size: 3,
                num_items: 6,
                buffer_vertices: vertices,
            },
        };

        let attribute_locations = AttributeLocationsLocal {
            vertex_position: gl.get_attrib_location(&program, "aVertexPosition"),
        };
        gl.enable_vertex_attrib_array(attribute_locations.vertex_position as u32);
        let uniform_locations = UniformLocations {
            projection_matrix: gl.get_uniform_location(&program, "uPMatrix").unwrap(),
            model_view_matrix: gl.get_uniform_location(&program, "uMVMatrix").unwrap(),
        };
        Self {
            ship,
            input,
            transform,
            program,
            attribute_locations,
            uniform_locations,
        }
    }

    fn input(&mut self) -> &mut UserInput {
        &mut self.input
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
    fn set_input(&mut self, input: UserInput) {
        self.input = input;
    }

    fn draw_scene(&mut self, gl: &GL, canvas: &CanvasData) {
        gl.clear_color(0., 0., 0., 1.);
        gl.clear_depth(1.);
        gl.enable(GL::DEPTH_TEST);
        gl.depth_func(GL::LEQUAL);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let z_near: f32 = 0.1;
        let z_far: f32 = 100.0;

        let projection_matrix =
            glm::perspective(canvas.get_aspect(), canvas.get_fov(), z_near, z_far);

        // TODO Apply movement changes to Spaceship
        self.ship.draw(
            gl,
            &self.attribute_locations,
            &self.uniform_locations,
            projection_matrix,
        );
    }

    fn update(&mut self, delta_time: f32) {
        if self.input.keyboard_a {
            self.ship.rotation += -0.01 * delta_time;
        }
        if self.input.keyboard_d {
            self.ship.rotation += 0.01 * delta_time;
        }
        if self.input.keyboard_w {
            self.ship.velocity += glm::vec3(0.00, 0.01, 0.0);
        }
        if self.input.keyboard_s {
            self.ship.velocity -= glm::vec3(0.00, 0.01, 0.0);
        }
        self.ship.update(delta_time);
    }
}
