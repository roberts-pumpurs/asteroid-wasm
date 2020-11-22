pub mod shaders;
pub mod ship;
pub mod transform;
use crate::canvas::CanvasData;
use crate::input::UserInput;
use crate::programs::asteroid::ship::Asteroid;
use crate::transform::Transform as UserTransform;
use crate::utils::console_log;
use crate::RenderObjectTrait;
use core::f32::consts::PI;
use std::collections::HashMap;
use js_sys::Function;
use rand::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;

use self::ship::{Bullet, SpaceShip};

use super::box_2d::UniformLocations;

const Z_OFFSET: f32 = -10.;

#[derive(Debug, Clone)]
pub struct GameObject {
    pub radius: f32,
    pub angle: f32,
    pub speed: f32,
    pub position: bevy_math::Vec2,
    pub direction: bevy_math::Vec2,
    pub scale: bevy_math::Vec3,
    pub rotation: bevy_math::Mat3,
    pub transformation: transform::Transform,

    pub buffers: Drawable,
}

impl GameObject {
    pub fn new(buffers: Drawable, offset_z: f32) -> Self {
        let mut s = Self {
            radius: 0.,
            angle: 0.,
            speed: 0.,
            position: bevy_math::Vec2::new(0., 0.),
            direction: bevy_math::Vec2::new(0., 1.),
            scale: bevy_math::Vec3::new(1., 1., 1.),
            rotation: bevy_math::Mat3::identity(),
            transformation: transform::Transform::identity(),
            buffers,
        };
        s
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation = get_matrix_rotation(self.angle);

        // Update direction matrix
        let dir3 = self.rotation.mul_vec3(bevy_math::Vec3::new(0., 1., 0.));
        self.direction = get_vec2_from_vec3(&dir3);

        // Update translation matrix
        let velocity = self.direction * self.speed * delta_time;
        self.position += velocity;
    }

    pub fn does_overlap(obj1: &GameObject, obj2: &GameObject) -> bool {
        let overlap = GameObject::circles_overlap(obj1.position, obj1.radius, obj2.position, obj2.radius,);
        overlap
    }

    /* Helpers */
    fn circles_overlap(c1: bevy_math::Vec2, r1: f32, c2: bevy_math::Vec2, r2: f32) -> bool {
        let delta: bevy_math::Vec2 = c2 - c1;
        delta.length_squared() <= (r1 + r2).powi(2)
    }
}

pub struct AttributeLocationsLocal {
    pub vertex_position: i32,
}

#[derive(Debug, Clone)]
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
    pub asteroids: HashMap<u64, Asteroid>,
    pub input: UserInput,
    pub transform: UserTransform,
    pub score: u64,
    max_asteroid_id: u64,
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
            asteroids: HashMap::new(),
            max_asteroid_id: 0,
            score: 0,
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

        // console_log(&format!("asteroids {:?}", &self.asteroids.len()));
        for bullet in self.bullets.iter_mut() {
            bullet.0.draw(
                gl,
                &self.attribute_locations,
                &self.uniform_locations,
                projection_matrix,
                &canvas,
            )
        }
        for asteroid in self.asteroids.iter_mut() {
            asteroid.1.obj.draw(
                gl,
                &self.attribute_locations,
                &self.uniform_locations,
                projection_matrix,
                &canvas,
            )
        }
    }

    fn update(&mut self, delta_time: f32, gl: &GL, canvas: &CanvasData, update_score: &Function) {
        /* Keyboard event capture */
        if self.input.keyboard_a {
            self.ship.obj.angle += -5.;
        }
        if self.input.keyboard_d {
            self.ship.obj.angle += 5.;
        }
        if self.input.keyboard_w {
            self.ship.obj.speed += 0.0001;
        }
        if self.input.keyboard_s {
            self.ship.obj.speed -= 0.0001;
        }
        /* Generate bullets */
        if self.input.spacebar {
            if self.ship.last_shot > 300. {
                let mut bullet = Bullet::new(gl, Z_OFFSET);
                bullet.0.direction = self.ship.obj.direction;
                bullet.0.angle = self.ship.obj.angle;
                bullet.0.position = self.ship.obj.position;
                bullet.0.speed = 0.01;
                self.bullets.push(bullet);
                self.ship.last_shot = 0.
            }
        }
        /* Generate asteroids */

        let mut rng = rand::thread_rng();
        if self.asteroids.len() < 20 {
            const INIT_RADIUS: f32 = 1.;
            let mut asteroid = Asteroid::new(gl, Z_OFFSET, INIT_RADIUS);

            let rand_x = if rand::random() {
                // left side
                -8.
            } else {
                // right side
                8.
            };
            let rand_y = if rand::random() {
                // below
                -5.
            } else {
                // above
                5.
            };
            asteroid.obj.position = bevy_math::Vec2::new(rand_x, rand_y);
            asteroid.obj.speed = rng.gen_range(0.0008, 0.0015);
            asteroid.obj.scale = bevy_math::Vec3::new(
                INIT_RADIUS, INIT_RADIUS, INIT_RADIUS
            );
            asteroid.obj.direction =
                bevy_math::Vec2::new(rng.gen_range(1., 100.), rng.gen_range(1., 100.));
            asteroid.obj.angle = rng.gen_range(0, 360) as f32;
            self.max_asteroid_id += 1;

            self.asteroids.insert(self.max_asteroid_id, asteroid);
        }

        /* Despawn objects */
        // Bullets go out of range
        self.bullets.retain(|el| {
            !((el.0.position.y() / 11.).abs() + 0.6 > 1.)
                || ((el.0.position.x() / 11.).abs() * 1.6 > 1.)
        });
        // Bullets interact with asteroids
        let mut removable_bullets = vec![];
        let mut removable_asteroids = vec![];

        for bullet in &self.bullets {
            let mut drop_bullet = false;
            for asteroid in &self.asteroids {
                drop_bullet = GameObject::does_overlap(&bullet.0, &asteroid.1.obj);
                if drop_bullet {
                    self.score += 1;
                    let score = JsValue::from_f64(self.score as f64);
                    // Crash explicitly if cannot update global score
                    update_score.call1(&score, &score).unwrap();
                    removable_asteroids.push(asteroid);
                    break;
                }
            }
            if drop_bullet {
                removable_bullets.push(bullet as *const Bullet);
            }
        }
        self.bullets.retain(|b| {
            !removable_bullets.contains(&(b as *const Bullet))
        });

        // Split asteroids
        let mut destroyable_keys = vec![];
        let mut children_asteroids = HashMap::new();
        let mut local_max = self.max_asteroid_id.clone();
        let mut iterations: u64 = 0;
        removable_asteroids.iter().for_each(|(key, a)| {
            if a.obj.radius > 0.3 {
                let pieces = rng.gen_range(2, 4);
                for _ in 0..pieces {
                    let radius = a.obj.radius / pieces as f32;
                    let mut asteroid = Asteroid::new(gl, Z_OFFSET, radius);
                    // asteroid.obj.transformation = a.obj.transformation;
                    asteroid.obj.position = a.obj.position.clone();
                    // asteroid.obj.position *= radius;
                    asteroid.obj.radius = radius;
                    asteroid.obj.speed = rng.gen_range(0.001, 0.005);
                    asteroid.obj.direction = bevy_math::Vec2::new(rng.gen_range(1., 100.), rng.gen_range(1., 100.));
                    asteroid.obj.angle = rng.gen_range(0, 360) as f32;

                    iterations += 1;
                    let key = local_max + iterations;
                    children_asteroids.insert(key,asteroid);
                }
            }
            destroyable_keys.push(key.clone().clone());
        });
        self.max_asteroid_id += iterations;
        self.asteroids.extend(children_asteroids);

        // Clean up asteroids
        self.asteroids.iter().for_each(|(key, el)| {
            // Asteroids go out of range
            if (el.obj.position.y().abs() > 8.) || (el.obj.position.x().abs() > 11.) {
                destroyable_keys.push(key.clone());
            }
        });

        for k in &destroyable_keys {
            self.asteroids.remove(k);
        }


        /* Position updates */
        self.ship.update(delta_time);

        for bullet in self.bullets.iter_mut() {
            bullet.update(delta_time);
        }
        for asteroid in self.asteroids.iter_mut() {
            asteroid.1.update(delta_time);
        }

    }
}
