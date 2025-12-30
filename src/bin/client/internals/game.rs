use crate::vec3;
use crate::internals::math::{Vector3};
use crate::internals::renderer::{Renderer};

use glfw::{Context, Action, Key, fail_on_errors};

use crate::{GAME_NAME, WINDOW_SIZE_X, WINDOW_SIZE_Y};


struct Player {
    position: Vector3,
}
impl Player {
    fn new() -> Self {
        Self{
            position: vec3!(0.0, 0.0, 0.0)
        }
    }
}

pub struct Game {
    player: Player,
    renderer: Renderer,
}
impl Game {
    pub fn new() -> Self {
        let player = Player::new();
        let renderer = Renderer::new();

        return Self{
            player,
            renderer,
        }
    }
    pub fn start(&self, ) -> Result<(), ()> {
        //init Window
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut window, events) = glfw.create_window(
            WINDOW_SIZE_X, WINDOW_SIZE_Y,
            GAME_NAME,
            glfw::WindowMode::Windowed,
        ).expect("Failed to create GLFW Window.");

        window.make_current();
        window.set_key_polling(true);

        self.renderer.init(&mut window);

        while !window.should_close() {
            
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                Self::process_event(event, &mut window)
            }
            self.renderer.render();
            window.swap_buffers();
        }
        
        return Ok(())
    }

    fn process_event(event: glfw::WindowEvent, window: &mut glfw::Window) {
        dbg!(&event);
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            },
            _ => {},
        }
    }
}