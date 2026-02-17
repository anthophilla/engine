use glfw::{Context, GlfwReceiver, WindowEvent};

use crate::{
    Crash,
    game::{
        GameAction,
        Input,
        settings::Settings
    }
};

pub enum WindowError{
    InitError,
    CreateError
}

pub enum WindowMode {
    Windowed,
    FullScreen,
}

//wrapper for glfw wrapper
pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub input: Input,
    ///for delta time
    last_time: f64,
}
impl Window {
    pub fn new(settings: &Settings) -> Result<Self, WindowError> {
        
        let mut glfw = 
            glfw::init_no_callbacks().map_err(|_| WindowError::InitError)?;
            //glfw::init(fail_on_errors!()).unwrap(); //dont use unwrap
        let (window, events) = glfw.create_window(
            settings.window_size.0, 
            settings.window_size.1,
            settings.game_title,
            match settings.window_mode {
                WindowMode::Windowed => glfw::WindowMode::Windowed,
                WindowMode::FullScreen => todo!("no fullscreen yet")
            }
        ).ok_or(WindowError::CreateError)?;

        let input = Input::from_settings(&settings.input_settings);

        Ok(Self { glfw, window, events, input, last_time: 0.0 })
    }

    pub fn start(&mut self, cursor_mode: glfw::CursorMode) {
        self.window.make_current();
        self.window.set_key_polling(true);
        self.window.set_size_polling(true);

        self.set_cursor_mode(cursor_mode);
    }
    pub fn set_cursor_mode(&mut self, cursor_mode: glfw::CursorMode) {
        self.window.set_cursor_mode(cursor_mode);
    }
    pub fn set_should_close(&mut self, close: bool) {
        self.window.set_should_close(close);
    }
    pub fn swap_buffers(&mut self) { self.window.swap_buffers(); }
    pub fn should_close(&self) -> bool { self.window.should_close() }
    pub fn make_current(&mut self) { self.window.make_current(); }

    fn set_delta_time(&mut self) {
        let current_time = self.glfw.get_time();
        self.input.delta_time = (self.last_time-current_time) as f32;
        self.last_time = current_time;
    }

    pub fn process_input(&mut self) -> Result<GameAction, Crash> {
        self.glfw.poll_events();
        self.set_delta_time();
        Ok(self.input.process(&self.events)?) 
    }
}