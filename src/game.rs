use crate::renderer::objects::Rectangle;
use crate::vector;
use crate::{
    math::{
        Vector,
        Quaternion,
    },
    renderer::{
        Renderer,
        objects::Cube,
        textures::Texture,
    },
    Error,
};

use glfw::{Action, Context, GlfwReceiver, Key, WindowEvent, fail_on_errors};

use crate::{GAME_NAME, WINDOW_SIZE_X, WINDOW_SIZE_Y};

struct Player {
    position: Vector<3>,
}
impl Player {
    fn new() -> Self {
        Self{
            position: vector!(0.0, 0.0, 0.0)
        }
    }
}

pub struct Game {
    player: Player,
    renderer: Renderer,
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    cubes: Vec<Cube>
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

        let cubes = vec![
            Cube::new(
                (0.5, 0.5, 0.5),
                vector!(0.0, 0.0, 1.0),
                Quaternion::from_angle_vect(0.0, vector!(1.0, 0.0, 0.0)),
                vector!(1.0, 0.0, 0.0, 1.0),
                vec![
                    Texture::from_file("src/textures/container.jpg").unwrap(),
                    Texture::from_file("src/textures/awesomeface.png").unwrap()
                ],
                gl::DYNAMIC_DRAW
            )
        ];

        return Self{
            player,
            renderer,
            glfw,
            window,
            events,
            cubes
        }
    }
    pub fn start(&mut self) -> Result<(), Error> {
        
        self.window.make_current();
        self.window.set_key_polling(true);
        self.window.set_size_polling(true);

        while !self.window.should_close() {
            
            self.process_events();
            self.renderer.render(vec![
                &mut self.cubes[0].mesh,
                // &mut Rectangle::new(
                //     (0.5, 0.5),
                //     vector!(0.0, 0.0, 1.0),
                //     Quaternion::from_angle_vect(0.0, vector!(0.0, 0.0, 0.0)),
                //     vector!(1.0, 0.0, 0.0, 1.0),
                //     vec![
                //         Texture::from_file("src/textures/container.jpg").unwrap(),
                //         Texture::from_file("src/textures/awesomeface.png").unwrap()
                //     ],
                //     gl::STATIC_DRAW
                // ).mesh
            ], self.glfw.get_time())?;
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