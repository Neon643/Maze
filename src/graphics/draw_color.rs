#[derive(Clone, Copy)]
pub struct DrawColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl DrawColor {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const DARK_GRAY: Self = Self::new(0.2, 0.2, 0.2, 1.0);
    pub const LIGHT_GRAY: Self = Self::new(0.8, 0.8, 0.8, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn r(&self) -> f32 {
        self.r
    }
    pub fn g(&self) -> f32 {
        self.g
    }
    pub fn b(&self) -> f32 {
        self.b
    }
    pub fn a(&self) -> f32 {
        self.a
    }
}
