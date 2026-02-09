use crate::{renderer::Camera, math::Vector, vector};

pub struct Player {
    pub camera: Camera,
}
impl Player {
    fn new() -> Self {
        let camera = Camera::new(
            vector!(0.0, 0.0, 0.0),
            90.0,
            1.0,
            100.0,
    );
        Self { camera }
    }
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}