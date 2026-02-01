use glfw::{Action, Key};

mod input;
mod settings;
pub use input::Input;
pub use settings::{Settings, InputSettings};

use crate::{
    Crash,
    renderer::{Camera, Renderer, Window, mesh::StaticMesh, window}
};

//any error that is not engines
pub enum GameError{
    Other(String)
}

pub enum GameAction {
    None,
    Exit,
    LoadScene(Scene),
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
            self.window.process_input()?;

            //run update functions
            for fun in &update_functions {
                let action = fun(&mut self.scene, &self.window.input)?;
                self.handle_action(action);
            }

            let static_meshes = &self.scene.world;
            self.renderer.render(static_meshes)?;
            self.window.swap_buffers()
        }
        Ok(())
    }

    fn handle_action(&mut self, action: GameAction) {
        match action {
            GameAction::Exit => self.quit(),
            GameAction::LoadScene(scene) => self.load_scene(scene),
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