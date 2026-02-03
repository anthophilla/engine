mod input;
mod settings;
use std::arch::x86_64;

pub use input::Input;
pub use settings::{Settings, InputSettings};

use crate::{
    Crash,
    renderer::{Camera, RenderError, Renderer, Window, mesh::StaticMesh},
    math::Vector,
    vector
};

//any error that is not engines
pub enum GameError{
    Other(String),
    Engine(Crash)
}
impl From<RenderError> for GameError {
    fn from(value: RenderError) -> Self {
        Self::Engine(value.into())
    }
}

pub enum GameAction {
    None,
    Exit,
    LoadScene(Scene),
    Resize(u32, u32),
}

pub struct Entity {}

pub struct Player {
    camera: Camera,
}
impl Player {
    fn new() -> Self {
        let camera = Camera::new(
            vector!(0.0, 0.0, 0.0),
            90.0,
            1.0,
            100.0,
    );
        Self { camera }
    }
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Scene {
    entities: Vec<Entity>,
    world: Vec<StaticMesh>, //change later
    player: Player
}
impl Scene {
    pub fn new(entities: Vec<Entity>, world: Vec<StaticMesh>, player: Player) -> Self {
        Self { entities, world, player }
    }
    //pub fn load(&self)
}
impl Default for Scene {
    fn default() -> Self {
        Self::new(vec![], vec![], Player::default())
    }
}

pub struct Game {
    scene: Scene,
    renderer: Renderer,
    settings: Settings,
    window: Window
}

impl Game {
    pub fn init(settings: Settings) -> Result<Self, Crash> {
        let mut window = Window::new(&settings)?;
        window.make_current();
        
        let renderer = Renderer::init(&mut window.window, &settings)?;

        Ok(Self {
            renderer,
            scene: Scene::default(),
            settings,
            window
        })
    }

    pub fn start(
        &mut self,
        start_functions: Vec<fn(&mut Scene, &Input) -> Result<GameAction, GameError>>,
        update_functions: Vec<fn(&mut Scene, &Input) -> Result<GameAction, GameError>>
    ) -> Result<(), Crash> {
        self.window.start(glfw::CursorMode::Normal);

        //run start functions
        for fun in &start_functions {
            let action = fun(&mut self.scene, &self.window.input)?;
            self.handle_action(action);
        }

        while !self.window.should_close() {
            let input_action = self.window.process_input()?;
            self.handle_action(input_action);

            //run update functions
            for fun in &update_functions {
                let action = fun(&mut self.scene, &self.window.input)?;
                self.handle_action(action);
            }

            let static_meshes = &self.scene.world;
            self.renderer.render(&self.scene.player.camera, static_meshes)?;
            self.window.swap_buffers()
        }
        Ok(())
    }

    fn handle_action(&mut self, action: GameAction) {
        match action {
            GameAction::Exit => self.quit(),
            GameAction::LoadScene(scene) => self.load_scene(scene),
            GameAction::Resize(x, y) => {
                self.scene.player.camera.change_aspect_ratio(x as f32 / y as f32);
                self.renderer.resize((x, y))
            },
            GameAction::None => {},
        }
    }

    fn quit(&mut self) {
        self.window.set_should_close(true);
    }

    fn load_scene(&mut self, scene: Scene) {
        self.scene = scene
    }
}