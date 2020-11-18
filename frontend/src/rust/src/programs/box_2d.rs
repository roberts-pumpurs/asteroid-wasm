use crate::utils::console_log;
use crate::input::UserInput;
use nalgebra_glm as glm;
// use std::time::{SystemTime, UNIX_EPOCH};
use js_sys::Date;

use super::{
    colors::SingleColor,
    cube::{plane::Plane2D, point::Point2D},
};
use crate::canvas::CanvasData;
use crate::transform::Transform;
use crate::RenderObjectTrait;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

pub struct AttributeLocations {
    pub vertex_position: i32,
    pub vertex_color: i32,
}

pub struct UniformLocations {
    pub projection_matrix: WebGlUniformLocation,
    pub model_view_matrix: WebGlUniformLocation,
}

pub struct Box2D {
    buffer_vertices: WebGlBuffer,
    buffer_colors: WebGlBuffer,
    program: WebGlProgram,
    attribute_locations: AttributeLocations,
    uniform_locations: UniformLocations,
    pub transform: Transform,
    pub input: UserInput,
    last_rotation: f64,
    vertices: Plane2D,
    colors: [SingleColor; 4],
    square_rotation: f64,
}

impl Box2D {
    fn init_buffers(
        gl: &GL,
        vertices: &Vec<f32>,
        colors: &[SingleColor; 4],
    ) -> (WebGlBuffer, WebGlBuffer) {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        let color_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));

        unsafe {
            let mut returnable: Vec<f32> = vec![];

            colors.iter().for_each(|p| {
                p.as_array().iter().for_each(|col| {
                    returnable.push(col.clone().clone());
                });
            });
            let colors_array = js_sys::Float32Array::view(&returnable);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors_array, GL::STATIC_DRAW);
        }
        (position_buffer, color_buffer)
    }
}

impl RenderObjectTrait for Box2D {
    fn new(gl: &GL, program: WebGlProgram, transform: Transform) -> Box2D {
        let attribute_locations = AttributeLocations {
            vertex_position: gl.get_attrib_location(&program, "aVertexPosition"),
            vertex_color: gl.get_attrib_location(&program, "aVertexColor"),
        };
        let uniform_locations = UniformLocations {
            projection_matrix: gl
                .get_uniform_location(&program, "uProjectionMatrix")
                .unwrap(),
            model_view_matrix: gl
                .get_uniform_location(&program, "uModelViewMatrix")
                .unwrap(),
        };

        let input = UserInput::new();

        let vertices = Plane2D::new(
            Point2D::new(-1., 1.),
            Point2D::new(1., 1.),
            Point2D::new(-1., -1.),
            Point2D::new(1., -1.),
        );
        let colors: [SingleColor; 4] = [
            SingleColor::new(1., 0., 0., 1.),
            SingleColor::new(1., 0.5, 1., 1.),
            SingleColor::new(1., 1., 0.5, 1.),
            SingleColor::new(0., 1., 1., 1.),
        ];
        let buffer = Box2D::init_buffers(&gl, &vertices.points_as_array(), &colors);

        Box2D {
            vertices,
            buffer_vertices: buffer.0,
            buffer_colors: buffer.1,
            attribute_locations,
            uniform_locations,
            program,
            transform,
            input,
            colors,
            square_rotation: 0.,
            last_rotation: Date::now() as f64,
        }
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
        let mut empty_matrix = glm::mat4x4(
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        empty_matrix.fill_with_identity();
        let translation_vector = glm::vec3(
            self.input.mouse_x_centered / 100.,
            self.input.mouse_y_centered / 100.,
            self.transform.get_trans_z(),
        );
        let mut model_view_matrix = glm::translate(&empty_matrix, &translation_vector);

        {
            // Perform rotation
            let rotation_vector = glm::vec3(0., 0., 1.);
            let now = Date::now();
            self.square_rotation += (now - self.last_rotation) * 0.001;
            self.last_rotation = now;

            model_view_matrix = glm::rotate_normalized_axis(
                &model_view_matrix,
                self.square_rotation.clone() as f32,
                &rotation_vector,
            );
        }

        {
            // Set vertices
            let number_components = 2;
            let buffer_type = GL::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer_vertices));
            gl.vertex_attrib_pointer_with_i32(
                self.attribute_locations.vertex_position as u32,
                number_components,
                buffer_type,
                normalize,
                stride,
                offset,
            );
            gl.enable_vertex_attrib_array(self.attribute_locations.vertex_position as u32);
        }

        {
            // Set colours
            let number_components = 4;
            let buffer_type = GL::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer_colors));
            gl.vertex_attrib_pointer_with_i32(
                self.attribute_locations.vertex_color as u32,
                number_components,
                buffer_type,
                normalize,
                stride,
                offset,
            );
            gl.enable_vertex_attrib_array(self.attribute_locations.vertex_color as u32);
        }

        gl.use_program(Some(&self.program));
        let transpose = false;
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_locations.projection_matrix),
            transpose,
            projection_matrix.as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_locations.model_view_matrix),
            transpose,
            model_view_matrix.as_slice(),
        );

        let offset = 0;
        let vertex_count = 4;
        gl.draw_arrays(GL::TRIANGLE_STRIP, offset, vertex_count);
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
}
