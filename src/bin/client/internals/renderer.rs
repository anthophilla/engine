use glfw::{Glfw, Context}

pub struct Renderer {
    glfw: mut Glfw
}
impl Renderer {
    fn init() -> Self {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap()
    }
}