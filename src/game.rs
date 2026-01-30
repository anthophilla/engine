use glfw::{Action, Key};

mod input;
mod settings;
pub use input::Input;
pub use settings::{Settings, InputSettings};

use crate::{
    Crash,
    renderer::{Camera, Renderer, Window, window}
};

//any error that is not engines
pub enum GameError{
    Custom(String)
}

pub enum GameAction {
    Exit,
}

pub struct Entity {}

pub struct Player {
    camera: Camera,
}
impl Player {
    fn new() -> Self {
        Self { camera: Camera::new() }
    }
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Scene {
    entities: Vec<Entity>,
    player: Player
}
impl Scene {
    pub fn new(entities: Vec<Entity>, player: Player) -> Self {
        Self { entities, player }
    }
}
impl Default for Scene {
    fn default() -> Self {
        Self::new(vec![], Player::default())
    }
}

pub struct Game {
    scene: Scene,
    renderer: Renderer,
    settings: Settings,
    window: Window
}

impl Game {
    pub fn init(scene: Scene, settings: Settings) -> Result<Self, Crash> {
        let window = Window::new(&settings)?;
        
        let renderer = Renderer::init()?;

        let input = Input::from_settings(&settings.input_settings);

        Ok(Self {
            renderer,
            scene,
            settings,
            window
        })
    }

    pub fn start(&mut self, update_functions: Vec<fn(&mut Scene, &Input) -> Result<Option<GameAction>, GameError>>) -> Result<(), Crash> {
        self.window.start(glfw::CursorMode::Normal);

        while !self.window.should_close() {
            self.window.process_input()?;
            for fun in &update_functions {
                match fun(&mut self.scene, &self.window.input)? {
                    Some(action) => self.handle_action(&action),
                    None => {},
                }
            }

            self.window.swap_buffers()
        }
        Ok(())
    }

    fn handle_action(&mut self, action: &GameAction) {
        match action {
            GameAction::Exit => self.quit(),
        }
    }

    fn quit(&mut self) {
        self.window.set_should_close(true);
    }
}