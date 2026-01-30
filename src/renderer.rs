pub mod camera;
pub mod window;
pub use camera::Camera;
pub use window::{Window, WindowError, WindowMode};

use crate::{
    game::Settings,
    math::{Vector, Color},
    vector
};

const BACKGROUND_COLOR: Color = vector!(0.5, 0.0, 0.0, 1.0);

pub enum RenderError {
    InitError(String)
}

pub struct Renderer {}

impl Renderer {
    pub fn init(window: &mut glfw::Window, settings: &Settings) -> Result<Self, RenderError> {
        //bad practise unwrap
        gl::load_with(
            |s| window.get_proc_address(s).unwrap() as *const _
        );

        Self::set_viewport(settings.window_size.0 as i32, settings.window_size.1 as i32);
        
        Ok(Self {

        })
    }

    pub fn render(&self) -> Result<(), RenderError> {
        self.clear_color(BACKGROUND_COLOR);
        self.clear();

        Ok(())
    }

    fn set_viewport(x: i32, y: i32) { unsafe { gl::Viewport(0, 0, x, y); } }
    fn clear_color(&self, color: Color) {unsafe {
        gl::ClearColor(color.0[0], color.0[1], color.0[2], color.0[3]);
    }}
    fn clear(&self) {unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }}
}