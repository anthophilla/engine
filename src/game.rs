use crate::math::Vector3;
use crate::renderer::camera::Camera;
use crate::renderer::objects::{Rectangle, StaticMesh};
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

//will change a lot
#[derive(Default)]
pub struct Input {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
    pub shift: bool,
}

#[derive(Debug)]
pub struct Player {
    world_position: Vector3,
    pub camera: Camera,
    pub speed: f32,
}
impl Player {
    pub fn new() -> Self {
        Self{
            world_position: vector!(0.0, 0.0, 0.0),
            camera: Camera::new(vector!(0.0, 0.0, -2.0)),
            speed: 10.0
        }
    }
    pub fn translate(&mut self, offset: Vector3) { self.world_position = self.world_position+offset }
    pub fn get_camera_world_position(&self) -> Vector3 { self.world_position+self.camera.local_position }
}

pub struct Game {
    pub player: Player,
    renderer: Renderer,
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    meshes: Vec<StaticMesh>,
    pub delta_time: f64,
    input: Input,
}
impl Game {
    pub fn new(player: Player) -> Self {
        //create a window
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut window, events) = glfw.create_window(
            WINDOW_SIZE_X, WINDOW_SIZE_Y,
            GAME_NAME,
            glfw::WindowMode::Windowed,
        ).expect("Failed to create GLFW Window.");
        
        let renderer = Renderer::init( &mut window);

        let meshes = vec![
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
            ).mesh,
            Cube::new(
                (0.5, 0.5, 0.5),
                vector!(0.0, 1.0, -2.0),
                Quaternion::from_angle_vect(0.0, vector!(1.0, 0.0, 0.0)),
                vector!(1.0, 0.0, 0.0, 1.0),
                vec![
                    Texture::from_file("src/textures/container.jpg").unwrap(),
                    Texture::from_file("src/textures/awesomeface.png").unwrap()
                ],
                gl::DYNAMIC_DRAW
            ).mesh
        ];

        return Self{
            player,
            renderer,
            glfw,
            window,
            events,
            meshes,
            delta_time: 0.0,
            input: Input::default(),
        }
    }
    pub fn start(&mut self, update: fn(&Input, &mut Player, f32)) -> Result<(), Error> {
        
        self.window.make_current();
        self.window.set_key_polling(true);
        self.window.set_size_polling(true);

        let mut last_time: f64 = 0.0;
        while !self.window.should_close() {
            let rot = Quaternion::from_angle_vect((self.glfw.get_time() as f32)*10.0, vector!(1.0, 0.0, 1.0));

            for mesh in self.meshes.iter_mut() { mesh.set_rotation(rot);}

            self.process_events();
            self.renderer.render(&self.meshes, &mut self.player)?;
            self.window.swap_buffers();

            let current_time = self.glfw.get_time();
            self.delta_time = last_time-current_time;
            last_time = current_time;

            update(&self.input, &mut self.player, (self.delta_time as f32));
        }
        
        return Ok(())
    }

    fn process_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => {
                    self.renderer.switch_wireframe()
                },
                glfw::WindowEvent::Size(x, y) => {
                    self.player.camera.set_aspect_ratio(x as f32/y as f32);
                    self.renderer.resize(x, y)
                },

                glfw::WindowEvent::Key(key, _, action, _) => {
                    match (key, action) {
                        (Key::W, Action::Press) => self.input.w = true,
                        (Key::W, Action::Release) => self.input.w = false,

                        (Key::S, Action::Press) => self.input.s = true,
                        (Key::S, Action::Release) => self.input.s = false,

                        (Key::A, Action::Press) => self.input.a = true,
                        (Key::A, Action::Release) => self.input.a = false,

                        (Key::D, Action::Press) => self.input.d = true,
                        (Key::D, Action::Release) => self.input.d = false,

                        (Key::Space, Action::Press) => self.input.space = true,
                        (Key::Space, Action::Release) => self.input.space = false,

                        (Key::LeftShift, Action::Press) => self.input.shift = true,
                        (Key::LeftShift, Action::Release) => self.input.shift = false,

                        _ => {},
                    }
                }

                _ => {},
            }
        }
    }
}