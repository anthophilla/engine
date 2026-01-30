pub mod camera;
pub mod window;
pub use camera::Camera;
pub use window::{Window, WindowError, WindowMode};

pub enum RenderError {
    InitError
}

pub struct Renderer {}

impl Renderer {
    pub fn init() -> Result<Self, RenderError> {
        
        
        Ok(Self {

        })
    }
}