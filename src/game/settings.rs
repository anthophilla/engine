use crate::renderer::window::WindowMode;

//change name later
type Keys = (glfw::Key, glfw::Key);
pub struct InputSettings {
    pub forward: Keys,
    pub right: Keys,

    pub exit: glfw::Key,
}
impl Default for InputSettings {
    fn default() -> Self {
        Self {
            forward: (glfw::Key::W, glfw::Key::S),
            right: (glfw::Key::D, glfw::Key::A),

            exit: glfw::Key::Escape,
        }
    }
}

pub struct Settings {
    pub window_size: (u32, u32),
    pub window_mode: WindowMode,
    pub game_title: &'static str,

    pub input_settings: InputSettings
}