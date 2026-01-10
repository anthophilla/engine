use crate::vec3;
use crate::{
    math::{Vector3, Triangle},
    renderer::{Renderer},
    Error,
};

use glfw::{Action, Context, GlfwReceiver, Key, WindowEvent, fail_on_errors};

use crate::{GAME_NAME, WINDOW_SIZE_X, WINDOW_SIZE_Y};

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

const TEXTURE_COORDS: [[f32; 2]; 6] = [
    [1.0, 1.0],
    [1.0, 0.0],
    [0.0, 1.0],

    [1.0, 0.0],
    [0.0, 0.0],
    [0.0, 1.0],
];

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
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>
}
impl Game {
    pub fn new() -> Self {
        let player = Player::new();
        
        //create a window
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut window, events) = glfw.create_window(
            WINDOW_SIZE_X, WINDOW_SIZE_Y,
            GAME_NAME,
            glfw::WindowMode::Windowed,
        ).expect("Failed to create GLFW Window.");
        
        let renderer = Renderer::init( &mut window);

        return Self{
            player,
            renderer,
            glfw,
            window,
            events
        }
    }
    pub fn start(&mut self, ) -> Result<(), Error> {
        
        self.window.make_current();
        self.window.set_key_polling(true);
        self.window.set_size_polling(true);

        while !self.window.should_close() {
            
            self.process_events();
            self.renderer.render(vec![TEST_TRIANGLE1, TEST_TRIANGLE2], TEXTURE_COORDS)?;
            self.window.swap_buffers();
        }
        
        return Ok(())
    }

    fn process_events(&mut self,) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => {
                    self.renderer.switch_wireframe()
                },
                glfw::WindowEvent::Size(x, y) => self.renderer.resize(x, y),
                _ => {},
            }
        }
    }
}