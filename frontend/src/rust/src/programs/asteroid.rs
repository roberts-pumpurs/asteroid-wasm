pub mod shaders;
pub mod ship;
pub mod transform;
use crate::canvas::CanvasData;
use crate::input::UserInput;
use crate::transform::Transform as UserTransform;
use crate::utils::console_log;
use crate::RenderObjectTrait;
use core::f32::consts::PI;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;

use self::ship::{Bullet, SpaceShip};

use super::box_2d::UniformLocations;

const Z_OFFSET: f32 = -10.;

pub struct GameObject {
    pub angle: f32,
    pub speed: f32,
    pub position: bevy_math::Vec2,
    pub direction: bevy_math::Vec2,
    pub scale: bevy_math::Vec3,
    pub rotation: bevy_math::Mat3,
    pub translation: bevy_math::Mat3,
    pub transformation: transform::Transform,

    pub buffers: Drawable,
}

impl GameObject {
    pub fn new(buffers: Drawable, offset_z: f32) -> Self {
        let mut s = Self {
            angle: 0.,
            speed: 0.,
            position: bevy_math::Vec2::new(0., 0.),
            direction: bevy_math::Vec2::new(0., 1.),
            scale: bevy_math::Vec3::new(1., 1., 1.),
            rotation: bevy_math::Mat3::identity(),
            translation: bevy_math::Mat3::identity(),
            transformation: transform::Transform::identity(),
            buffers,
        };
        // s.translation .translate(bevy_math::vec3(0., 0., -6.));
        s
        // TODO Perform Translation by Z axis
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

pub fn get_matrix_rotation(theta: f32) -> bevy_math::Mat3 {
    let theta_rad = theta * PI / 180.;
    let theta_cos = theta_rad.cos();
    let theta_sin = theta_rad.sin();
    bevy_math::Mat3::from_cols(
        bevy_math::Vec3::new(theta_cos, -theta_sin, 0.),
        bevy_math::Vec3::new(theta_sin, theta_cos, 0.),
        bevy_math::Vec3::new(0., 0., 1.),
    )
}

pub fn get_vec2_from_vec3(dir3: &bevy_math::Vec3) -> bevy_math::Vec2 {
    dir3.truncate()
}
impl GameObject {
    pub fn draw(
        &mut self,
        gl: &GL,
        attribute_locations: &AttributeLocationsLocal,
        uniform_locations: &UniformLocations,
        projection_matrix: bevy_math::Mat4,
        canvas: &CanvasData,
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

        //  --------- NEW --------- //
        let theta_rad = self.angle * PI / 180.;
        let rot = bevy_math::Quat::from_axis_angle(bevy_math::Vec3::new(0., 0., -1.), theta_rad);

        // Set factual values
        self.transformation.set_translation(bevy_math::Vec3::new(
            self.position.x(),
            self.position.y(),
            Z_OFFSET,
        ));
        self.transformation.set_rotation(rot);
        self.transformation.set_non_uniform_scale(self.scale);

        gl.uniform_matrix4fv_with_f32_array(
            Some(&uniform_locations.model_view_matrix),
            false,
            // self.model_view_matrix.as_slice(),
            &self.transformation.value().to_cols_array(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&uniform_locations.projection_matrix),
            false,
            &projection_matrix.to_cols_array(),
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
    pub transform: UserTransform,
    // GL
    program: WebGlProgram,
    attribute_locations: AttributeLocationsLocal,
    uniform_locations: UniformLocations,
}

impl RenderObjectTrait for AsteroidCanvas {
    fn new(gl: &GL, program: WebGlProgram, transform: UserTransform) -> Self
    where
        Self: Sized,
    {
        // Store metadata
        let input = UserInput::new();
        let ship = SpaceShip::new(gl, Z_OFFSET);

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
    fn transform(&self) -> &UserTransform {
        &self.transform
    }
    fn set_transform(&mut self, transform: UserTransform) {
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
        // let z_near: f32 = 0.1;
        let z_far: f32 = 100.0;

        /*  -------- Construct projection matrix -------- */
        let f = 1. / (canvas.get_fov() / 2.).tan();
        let range_inv = 1. / (z_near - z_far);
        let projection_matrix = bevy_math::mat4(
            bevy_math::vec4(f / canvas.get_aspect(), 0., 0., 0.),
            bevy_math::vec4(0., f, 0., 0.),
            bevy_math::vec4(0., 0., (z_near + z_far) * range_inv, -1.),
            bevy_math::vec4(0., 0., z_near * z_far * range_inv * 2., 0.),
        );

        /* Draw elements */
        self.ship.obj.draw(
            gl,
            &self.attribute_locations,
            &self.uniform_locations,
            projection_matrix,
            &canvas,
        );

        // console_log(&format!("Bullets {:?}", &self.bullets.len()));
        for bullet in self.bullets.iter_mut() {
            bullet.0.draw(
                gl,
                &self.attribute_locations,
                &self.uniform_locations,
                projection_matrix,
                &canvas,
            )
        }
    }

    fn update(&mut self, delta_time: f32, gl: &GL, canvas: &CanvasData) {
        if self.input.keyboard_a {
            self.ship.obj.angle += -5.;
        }
        if self.input.keyboard_d {
            self.ship.obj.angle += 5.;
        }
        if self.input.keyboard_w {
            self.ship.obj.speed += 0.0005;
        }
        if self.input.keyboard_s {
            self.ship.obj.speed -= 0.0005;
        }
        if self.input.spacebar {
            if self.ship.last_shot > 1000. {
                let mut bullet = Bullet::new(gl, Z_OFFSET);
                bullet.0.direction = self.ship.obj.direction;
                bullet.0.angle = self.ship.obj.angle;
                bullet.0.position = self.ship.obj.position;
                bullet.0.speed = 0.01;
                self.bullets.push(bullet);
                self.ship.last_shot = 0.
            }
        }
        self.ship.update(delta_time);

        // console_log(&format!("self.ship.last_shot {}", self.ship.last_shot,));
        for bullet in self.bullets.iter_mut() {
            bullet.update(delta_time);
        }
        self.bullets.retain(|el| {
            !((el.0.position.y() / 11.).abs() + 0.6 > 1.)
                || ((el.0.position.x() / 11.).abs() * 1.6 > 1.)
        });
    }
}
