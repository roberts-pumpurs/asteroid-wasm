pub mod shaders;
pub mod ship;
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

use self::ship::{Bullet, SpaceShip};

use super::box_2d::UniformLocations;

const Z_OFFSET: f32 = -8.;

pub struct GameObject {
    pub velocity: glm::TVec3<f32>,
    pub position: glm::TVec3<f32>,
    pub model_view_matrix: glm::TMat4<f32>,
    pub rotation: f32,
    pub buffers: Drawable,
}

impl GameObject {
    pub fn new(buffers: Drawable, offset_z: f32) -> Self {

        let mut s = Self {
            position: glm::vec3(0., 0., 0.),
            velocity: glm::vec3(0., 0., 0.),
            model_view_matrix: glm::mat4(
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            ),
            rotation: 0.,
            buffers,
        };
        s.model_view_matrix.fill_with_identity();
        s.model_view_matrix =
            glm::translate(&s.model_view_matrix, &glm::vec3(0., 0., offset_z));
        s
    }
}

pub struct AttributeLocationsLocal {
    pub vertex_position: i32,
}

pub struct Drawable {
    pub item_size: i32,
    pub num_items: i32,
    pub buffer_vertices: WebGlBuffer,
}

impl Drawable {
    pub fn new(item_size: i32, num_items: i32, buffer_vertices: WebGlBuffer) -> Self {
        Self {
            item_size,
            num_items,
            buffer_vertices,
        }
    }

}

pub fn get_matrix_rotation(theta: f32) -> glm::Mat3 {
    let theta_rad = theta * PI / 180.;
    let theta_cos = theta_rad.cos();
    let theta_sin = theta_rad.sin();
    glm::mat3(
        theta_cos, -theta_sin, 0., theta_sin, theta_cos, 0., 0., 0., 1.,
    )
}

impl GameObject {
    pub fn draw(
        &mut self,
        gl: &GL,
        attribute_locations: &AttributeLocationsLocal,
        uniform_locations: &UniformLocations,
        projection_matrix: glm::TMat4<f32>,
    ) {
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
        let rot = get_matrix_rotation(self.rotation);
        self.model_view_matrix = self.model_view_matrix * glm::mat3_to_mat4(&rot);

        /* Perform positional movement */
        let translation_vector = self.position;
        self.model_view_matrix = glm::translate(&self.model_view_matrix, &translation_vector);

        /* Apply screen wrapping */
        let x_coord = self.model_view_matrix.get(12).unwrap();
        let y_coord = self.model_view_matrix.get(13).unwrap();

        // console_log(&format!(
        //     "x_coord {:#?} y_coord {:#?}\n",
        //     &x_coord, &y_coord,
        // ));
        let mut new_mvm = self.model_view_matrix.clone_owned();
        if x_coord.abs() > 4. {
            new_mvm[12] = -x_coord;
        }
        if y_coord.abs() > 2.6 {
            new_mvm[13] = -y_coord;
        }
        self.model_view_matrix = new_mvm;

        gl.uniform_matrix4fv_with_f32_array(
            Some(&uniform_locations.model_view_matrix),
            false,
            self.model_view_matrix.as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&uniform_locations.projection_matrix),
            false,
            projection_matrix.as_slice(),
        );

        let offset = 0;
        gl.draw_arrays(GL::LINES, offset, self.buffers.num_items);
    }
}

pub struct AsteroidCanvas {
    // Game itself
    pub ship: SpaceShip,
    pub bullets: Vec<Bullet>,
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
        let mut ship = SpaceShip::new(gl, Z_OFFSET);

        let attribute_locations = AttributeLocationsLocal {
            vertex_position: gl.get_attrib_location(&program, "aVertexPosition"),
        };
        gl.enable_vertex_attrib_array(attribute_locations.vertex_position as u32);
        let uniform_locations = UniformLocations {
            projection_matrix: gl.get_uniform_location(&program, "uPMatrix").unwrap(),
            model_view_matrix: gl.get_uniform_location(&program, "uMVMatrix").unwrap(),
        };
        Self {
            bullets: vec![],
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

        /* Apply drag */
        self.ship.0.velocity = self.ship.0.velocity.scale(0.0005);
        self.ship.0.rotation = self.ship.0.rotation * 0.95;

        /* Draw elements */
        self.ship.0.draw(
            gl,
            &self.attribute_locations,
            &self.uniform_locations,
            projection_matrix,
        );

        console_log(&format!("Bullets {:?}", &self.bullets.len()));
        for bullet in self.bullets.iter_mut() {
            bullet.0.draw(
                gl,
                &self.attribute_locations,
                &self.uniform_locations,
                projection_matrix,
            )
        }
    }

    fn update(&mut self, delta_time: f32, gl: &GL) {
        if self.input.keyboard_a {
            self.ship.0.rotation += 0.01 * delta_time;
        }
        if self.input.keyboard_d {
            self.ship.0.rotation += -0.01 * delta_time;
        }
        if self.input.keyboard_w {
            self.ship.0.velocity += glm::vec3(0.00, 0.01, 0.0);
        }
        if self.input.keyboard_s {
            self.ship.0.velocity -= glm::vec3(0.00, 0.01, 0.0);
        }
        if self.input.spacebar {
            self.bullets.push(Bullet::new(gl, Z_OFFSET));
        }
        self.ship.update(delta_time);
        // for bullet in self.bullets.iter_mut() {
        //     bullet.update(delta_time);
        // }
    }
}
