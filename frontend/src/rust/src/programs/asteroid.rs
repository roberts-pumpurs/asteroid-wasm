use crate::canvas::CanvasData;
use crate::input::UserInput;
use crate::transform::Transform;
use crate::RenderObjectTrait;
use nalgebra_glm as glm;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

use super::{box_2d::AttributeLocations, box_2d::UniformLocations, colors::SingleColor};

pub struct Drawable {
    pub buffer_vertices: WebGlBuffer,
    pub buffer_colors: WebGlBuffer,
}

pub struct SpaceShip {
    pub speed: i32,
    pub direction: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub buffers: Drawable,
}

impl SpaceShip {
    pub fn draw(
        &self,
        gl: &GL,
        attribute_locations: &AttributeLocations,
        uniform_locations: &UniformLocations,
    ) {
        {
            // Set vertices
            let number_components = 2;
            let buffer_type = GL::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffers.buffer_vertices));
            gl.vertex_attrib_pointer_with_i32(
                attribute_locations.vertex_position as u32,
                number_components,
                buffer_type,
                normalize,
                stride,
                offset,
            );
            gl.enable_vertex_attrib_array(attribute_locations.vertex_position as u32);
        }

        let transpose = false;
        let offset = 0;
        let vertex_count = 4;
        gl.draw_arrays(GL::LINES, offset, vertex_count);
    }

    fn init_buffers(gl: &GL, vertices: Vec<f32>) -> (WebGlBuffer, WebGlBuffer) {
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
            let colors: [SingleColor; 4] = [
                SingleColor::new(1., 0., 0., 1.),
                SingleColor::new(1., 0.5, 1., 1.),
                SingleColor::new(1., 1., 0.5, 1.),
                SingleColor::new(0., 1., 1., 1.),
            ];
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

pub struct AsteroidCanvas {
    // Game itself
    pub ship: SpaceShip,
    pub input: UserInput,
    pub transform: Transform,
    // GL
    program: WebGlProgram,
    pub attribute_locations: AttributeLocations,
    pub uniform_locations: UniformLocations,
}

impl RenderObjectTrait for AsteroidCanvas {
    fn new(gl: &GL, program: WebGlProgram, transform: Transform) -> Self
    where
        Self: Sized,
    {
        // Store metadata
        let input = UserInput::new();

        // Construct spaceship
        let vertices: Vec<f32> = vec![-1., -1., 0., 1., 1., -1., -1., -1.];
        let (vertices, colors) = SpaceShip::init_buffers(gl, vertices);
        let ship = SpaceShip {
            speed: 1,
            direction: 1.,
            location_x: 0.,
            location_y: 0.,
            buffers: Drawable {
                buffer_vertices: vertices,
                buffer_colors: colors,
            },
        };

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
        // TODO Apply movement changes to Spaceship
        self.ship
            .draw(gl, &self.attribute_locations, &self.uniform_locations);

        let offset = 0;
        let vertex_count = 4;
        gl.draw_arrays(GL::TRIANGLE_STRIP, offset, vertex_count);
    }
}
