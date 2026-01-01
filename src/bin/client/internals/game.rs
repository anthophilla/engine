use crate::vec3;
use crate::internals::math::{Vector3, Triangle};
use crate::internals::renderer::{Renderer};

use glfw::{Context, Action, Key, fail_on_errors};

use crate::{GAME_NAME, WINDOW_SIZE_X, WINDOW_SIZE_Y};

const TEST_TRIANGLE: Triangle  = Triangle::new(
    vec3!(-0.5, -0.5, 0.0),
    vec3!(0.5, -0.5, 0.0),
    vec3!(0.0, 0.5, 0.0)
);

const TEST_TRIANGLE1: Triangle  = Triangle::new(
    vec3!(0.5, 0.5, 0.0),
    vec3!(0.5, -0.5, 0.0),
    vec3!(-0.5, 0.5, 0.0)
);
const TEST_TRIANGLE2: Triangle  = Triangle::new(
    vec3!(0.5, -0.5, 0.0),
    vec3!(-0.5, -0.5, 0.0),
    vec3!(-0.5, 0.5, 0.0)
);


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
    pub fn start(&mut self, ) -> Result<(), ()> {
        //init Window
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut window, events) = glfw.create_window(
            WINDOW_SIZE_X, WINDOW_SIZE_Y,
            GAME_NAME,
            glfw::WindowMode::Windowed,
        ).expect("Failed to create GLFW Window.");
        
        self.renderer.init(&mut window);
        
        window.make_current();
        window.set_key_polling(true);
        window.set_size_polling(true);

        while !window.should_close() {
            
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                self.process_event(event, &mut window)
            }
            self.renderer.render(vec![TEST_TRIANGLE1, TEST_TRIANGLE2]);
            window.swap_buffers();
        }
        
        return Ok(())
    }

    fn process_event(&self, event: glfw::WindowEvent, window: &mut glfw::Window) {
        dbg!(&event);
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            },
            glfw::WindowEvent::Size(x, y) => self.renderer.resize(x, y),
            _ => {},
        }
    }
}