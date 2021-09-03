pub struct SingleColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl SingleColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn as_array(&self) -> [&f32; 4] {
        [
            &self.r, //.clone(),
            &self.g, //.clone(),
            &self.b, //.clone(),
            &self.a, //.clone(),
        ]
    }
}
