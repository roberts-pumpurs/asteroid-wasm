pub mod plane;
pub mod point;
use crate::input::UserInput;
use crate::programs::box_2d::AttributeLocations;
use crate::programs::box_2d::UniformLocations;
use crate::programs::colors::SingleColor;
use crate::programs::cube::plane::Plane2D;
use crate::transform::Transform;
use crate::{utils::console_log, RenderObjectTrait};
use js_sys::Date;
use nalgebra_glm as glm;
use plane::Plane3D;
use point::Point3D;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;

pub struct Cube {
    pub sides: [Plane3D; 6],

    buffer_vertices: WebGlBuffer,
    buffer_colors: WebGlBuffer,
    buffer_indices: WebGlBuffer,
    program: WebGlProgram,
    attribute_locations: AttributeLocations,
    uniform_locations: UniformLocations,
    pub transform: Transform,
    pub input: UserInput,
    last_rotation: f64,
    square_rotation: f64,
    colors: [SingleColor; 6],
}

impl Cube {
    fn init_buffers(
        gl: &GL,
        vertices: &Vec<f32>,
        colors: &[SingleColor; 6],
    ) -> (WebGlBuffer, WebGlBuffer, WebGlBuffer) {
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
                for _ in 0..4 {
                    let vals = p.as_array();
                    returnable.push(vals.get(0).unwrap().clone().clone());
                    returnable.push(vals.get(1).unwrap().clone().clone());
                    returnable.push(vals.get(2).unwrap().clone().clone());
                    returnable.push(vals.get(3).unwrap().clone().clone());
                }
            });

            let colors_array = js_sys::Float32Array::view(&returnable);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors_array, GL::STATIC_DRAW);
        }

        let index_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        let indices = [
            0,  1,  2,      0,  2,  3,    // front
            4,  5,  6,      4,  6,  7,    // back
            8,  9,  10,     8,  10, 11,   // top
            12, 13, 14,     12, 14, 15,   // bottom
            16, 17, 18,     16, 18, 19,   // right
            20, 21, 22,     20, 22, 23,   // left
        ];

        unsafe {
            let indices_arr = js_sys::Uint16Array::view(&indices);
            gl.buffer_data_with_array_buffer_view(
                GL::ELEMENT_ARRAY_BUFFER,
                &indices_arr,
                GL::STATIC_DRAW,
            );
        }

        (position_buffer, color_buffer, index_buffer)
    }
}

impl RenderObjectTrait for Cube {
    fn new(gl: &GL, program: WebGlProgram, transform: Transform) -> Self {
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

        let scale = 0.5;
        let front = Plane3D::new(
            Point3D::new(-scale, -scale, scale),
            Point3D::new(-scale, scale, scale),
            Point3D::new(scale, scale, scale),
            Point3D::new(scale, -scale, scale),
        );
        let back = Plane3D::new(
            Point3D::new(-scale, -scale, -scale),
            Point3D::new(-scale, scale, -scale),
            Point3D::new(scale, scale, -scale),
            Point3D::new(scale, -scale, -scale),
        );
        let left = Plane3D::new(
            Point3D::new(-scale, -scale, -scale),
            Point3D::new(-scale, scale, -scale),
            Point3D::new(-scale, scale, scale),
            Point3D::new(-scale, -scale, scale),
        );
        let right = Plane3D::new(
            Point3D::new(scale, -scale, scale),
            Point3D::new(scale, scale, scale),
            Point3D::new(scale, scale, -scale),
            Point3D::new(scale, -scale, -scale),
        );
        let top = Plane3D::new(
            Point3D::new(-scale, scale, scale), // close left
            Point3D::new(-scale, scale, -scale),
            Point3D::new(scale, scale, -scale),
            Point3D::new(scale, scale, scale),
        );
        let bottom = Plane3D::new(
            Point3D::new(-scale, -scale, scale),
            Point3D::new(-scale, -scale, -scale),
            Point3D::new(scale, -scale, -scale),
            Point3D::new(scale, -scale, scale),
        );
        let sides: [Plane3D; 6] = [front, back, left, right, top, bottom];
        let mut vertices = vec![];

        sides.iter().for_each(|el| {
            el.points_as_array().iter().for_each(|p| {
                vertices.push(p.clone());
            });
        });

        // console_log(&format!("vertices {:?} {:?}", vertices, sides));

        let colors: [SingleColor; 6] = [
            SingleColor::new(1., 1., 1., 1.), // front
            SingleColor::new(1., 0., 0., 1.), // back
            SingleColor::new(0., 1., 0., 1.), // left
            SingleColor::new(0., 0., 1., 1.), // right
            SingleColor::new(1., 1., 0., 1.), // top
            SingleColor::new(1., 0., 1., 1.), // bottom
        ];
        let buffer = Cube::init_buffers(&gl, &vertices, &colors);

        Self {
            buffer_vertices: buffer.0,
            buffer_colors: buffer.1,
            buffer_indices: buffer.2,
            attribute_locations,
            uniform_locations,
            program,
            transform,
            input,
            colors,
            square_rotation: 0.,
            last_rotation: Date::now() as f64,
            sides,
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

    fn draw_scene(&mut self, gl: &GL, canvas: &crate::canvas::CanvasData) {
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
            // self.input.mouse_x_centered / 100.,
            0.,
            // self.input.mouse_y_centered / 100.,
            0.,
            self.transform.get_trans_z(),
        );
        let mut model_view_matrix = glm::translate(&empty_matrix, &translation_vector);

        {
            // Perform rotation
            let rotation_vector = glm::vec3(0., 0., 1.);

            let mut z_val_rot = 0.;
            if self.input.mouse_down {
                z_val_rot = self.input.mouse_x_centered * 0.01;
            }

            model_view_matrix = glm::rotate_normalized_axis(
                &model_view_matrix,
                z_val_rot,
                // self.square_rotation.clone() as f32,
                &rotation_vector,
            );

            let rotation_vector = glm::vec3(1., 0., 0.);

            model_view_matrix = glm::rotate_normalized_axis(
                &model_view_matrix,
                self.input.mouse_y_centered * 0.01,
                // self.square_rotation.clone() as f32,
                &rotation_vector,
            );
            let rotation_vector = glm::vec3(0., 1., 0.);

            model_view_matrix = glm::rotate_normalized_axis(
                &model_view_matrix,
                -self.input.mouse_x_centered * 0.01,
                &rotation_vector,
            );
        }

        {
            // Set vertices
            let number_components = 3;
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

        {
            // Draw triangles
            let vertex_count = 36;
            let buffer_type = GL::UNSIGNED_SHORT;
            let offset = 0;

            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.buffer_indices));
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
            gl.draw_elements_with_i32(GL::TRIANGLES, vertex_count, buffer_type, offset);

            let now = Date::now();
            self.square_rotation += (now - self.last_rotation) * 0.001;
            self.last_rotation = now;
        }
    }
}
