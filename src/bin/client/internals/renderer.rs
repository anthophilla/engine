//use gl::types::*;

pub struct Renderer;
impl Renderer {
    pub fn new() -> Self {
        return Self;
    }
    pub fn init(&self, window: &mut glfw::Window) {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);
        //gl::ClearColor::load_with(|s| window.get_proc_address(s).unwrap() as *const _);
    }

    pub fn render(&self) {
        let bg_color = crate::BACKGROUND_COLOR.as_tuple();
        unsafe { gl::ClearColor(bg_color.0, bg_color.1, bg_color.2, bg_color.3); }
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
    }
}