use crate::canvas::CanvasData;
use crate::utils::console_log;
use core::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct UserInput {
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_x_centered: f32,
    pub mouse_y_centered: f32,
    pub rotation_x_axis: f32,
    pub rotation_y_axis: f32,
}

impl UserInput {
    pub fn new() -> Self {
        Self {
            mouse_down: false,
            mouse_x: 0.,
            mouse_y: 0.,
            rotation_x_axis: 0.,
            rotation_y_axis: 0.,
            mouse_x_centered: 0.,
            mouse_y_centered: 0.,
        }
    }

    pub fn update_mouse_down(&mut self, x: f32, y: f32, is_down: bool) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_down = is_down;
    }

    pub fn update_mouse_position(&mut self, x: f32, y: f32, cd: &CanvasData) {
        // cd.width
        let inverted_y = cd.height - y;
        let x_delta = x - self.mouse_x;
        let y_delta = inverted_y - self.mouse_y;
        let rotation_x_delta = if self.mouse_down {
            PI * y_delta / cd.height
        } else {
            0.
        };
        let rotation_y_delta = if self.mouse_down {
            PI * x_delta / cd.width
        } else {
            0.
        };

        self.mouse_x_centered = - ((cd.width / 2.) - x);
        self.mouse_y_centered = (cd.height / 2.) - y;
        self.mouse_x = x;
        self.mouse_y = y;
        self.rotation_x_axis = self.rotation_x_axis + rotation_x_delta;
        self.rotation_y_axis = self.rotation_y_axis + rotation_y_delta;
    }
}
