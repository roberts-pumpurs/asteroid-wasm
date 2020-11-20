use crate::programs::asteroid;
use crate::programs::box_2d::Box2D;
use crate::programs::cube::Cube;
use crate::shaders::fragment::F_SHADER;
use crate::shaders::vertex::V_SHADER;
use crate::transform::Transform;
use crate::RenderableOption;
use crate::{canvas::CanvasData, programs::asteroid::AsteroidCanvas};
use crate::{
    gl_setup,
    utils::{console_log, link_program},
    RenderObjectTrait,
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext as GL};

#[wasm_bindgen]
pub struct GlClient {
    gl: GL,
    object: Option<Box<dyn RenderObjectTrait>>,
    pub is_ready: bool,
    master_canvas: HtmlCanvasElement,
    canvas: CanvasData,
}

#[wasm_bindgen]
impl GlClient {
    pub fn new(opt: RenderableOption, canvas: &CanvasData, transform: &Transform) -> Self {
        let canvas_el: HtmlCanvasElement = gl_setup::get_canvas(&canvas.get_canvas());
        let gl: GL = gl_setup::initialize_webgl_context(&canvas_el).unwrap();
        let mut client: GlClient = GlClient {
            canvas: canvas.clone(),
            gl,
            object: None,
            is_ready: false,
            master_canvas: canvas_el,
        };
        client.set_renderable(opt, transform);
        client
    }

    #[wasm_bindgen(constructor)]
    pub fn new_default(canvas: &CanvasData) -> Self {
        let canvas_el: HtmlCanvasElement = gl_setup::get_canvas(&canvas.get_canvas());
        let gl: GL = gl_setup::initialize_webgl_context(&canvas_el).unwrap();
        GlClient {
            canvas: canvas.clone(),
            gl,
            object: None,
            is_ready: false,
            master_canvas: canvas_el,
        }
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        match &mut self.object {
            Some(obj) => {
                obj.draw_scene(&self.gl, &self.canvas);
            }
            None => {
                console_log("Clearing the canvas");
                self.clear();
            }
        }
    }

    #[wasm_bindgen]
    pub fn set_renderable(&mut self, opt: RenderableOption, transform: &Transform) {
        console_log(&format!("Setting rendarble to {:?}", &opt));

        self.is_ready = false;

        let gl: GL = gl_setup::initialize_webgl_context(&self.master_canvas).unwrap();
        self.gl = gl;
        match opt {
            RenderableOption::Cube => {
                let program: WebGlProgram = link_program(&self.gl, &V_SHADER, &F_SHADER).unwrap();
                self.gl.use_program(Some(&program));
                let object: Box<Cube> =
                    Box::new(RenderObjectTrait::new(&self.gl, program, transform.clone()));
                self.object = Some(object);
            }
            RenderableOption::Box2D => {
                let program: WebGlProgram = link_program(&self.gl, &V_SHADER, &F_SHADER).unwrap();
                self.gl.use_program(Some(&program));
                let object: Box<Box2D> =
                    Box::new(RenderObjectTrait::new(&self.gl, program, transform.clone()));
                self.object = Some(object);
            }
            RenderableOption::Asteroid => {
                let program: WebGlProgram = link_program(
                    &self.gl,
                    &asteroid::shaders::V_SHADER,
                    &asteroid::shaders::F_SHADER,
                )
                .unwrap();
                self.gl.use_program(Some(&program));
                let object: Box<AsteroidCanvas> =
                    Box::new(RenderObjectTrait::new(&self.gl, program, transform.clone()));
                self.object = Some(object);
            }
        }
        self.is_ready = true;
    }

    fn clear(&self) {
        self.gl.clear_color(0., 0., 0., 1.);
        self.gl.clear_depth(1.);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    #[wasm_bindgen]
    pub fn get_transform(&mut self) -> Option<Transform> {
        match &mut self.object {
            Some(obj) => Some(obj.transform().clone()),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn set_transform(&mut self, new_transform: &Transform) {
        match &mut self.object {
            Some(obj) => {
                obj.set_transform(new_transform.clone());
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }

    #[wasm_bindgen]
    pub fn update_mouse_down(&mut self, x: f32, y: f32, is_down: bool) {
        match &mut self.object {
            Some(obj) => {
                obj.input().update_mouse_down(x, y, is_down);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }
    #[wasm_bindgen]
    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        match &mut self.object {
            Some(obj) => {
                obj.input().update_mouse_position(x, y, &self.canvas);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }

    #[wasm_bindgen]
    pub fn keyboard_space(&mut self, value: bool) {
        match &mut self.object {
            Some(obj) => {
                obj.input().update_spacebar(value);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }

    #[wasm_bindgen]
    pub fn keyboard_w(&mut self, value: bool) {
        match &mut self.object {
            Some(obj) => {
                obj.input().update_keyboard_w( value);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }
    #[wasm_bindgen]
    pub fn keyboard_a(&mut self, value: bool) {
        match &mut self.object {
            Some(obj) => {

                console_log(&"SOMETHING A");
                obj.input().update_keyboard_a( value);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }
    #[wasm_bindgen]
    pub fn keyboard_s(&mut self, value: bool) {
        match &mut self.object {
            Some(obj) => {
                obj.input().update_keyboard_s( value);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }
    #[wasm_bindgen]
    pub fn keyboard_d(&mut self, value: bool) {
        match &mut self.object {
            Some(obj) => {
                obj.input().update_keyboard_d( value);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }
    #[wasm_bindgen]
    pub fn update(&mut self, delta_time: f32) {
        match &mut self.object {
            Some(obj) => {
                obj.update(delta_time, &self.gl, &self.canvas);
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }
}
