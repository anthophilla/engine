pub mod input;
pub mod settings;
pub mod player;
pub mod scene;
pub mod object;

pub use input::Input;
pub use object::GameObject;

use crate::{
    Crash,
    game::{
        scene::{GameScene, LoadingScene, MenuScene, EmptyScene, Scene},
        settings::Settings
    },
    renderer::{RenderError, Renderer, Window},
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

pub enum GameState {
    Menu(Box<dyn MenuScene>),
    //Bad name
    Game(Box<dyn GameScene>),
    Loading(Box<dyn LoadingScene>),
}

pub enum GameAction {
    None,
    Exit,
    ///use also for changing game scenes
    ChangeGameState(GameState),
    Resize(u32, u32),
}

pub struct Game {
    state: GameState,
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
            state: GameState::Loading(Box::new(EmptyScene::new())),
            settings,
            window
        })
    }

    pub fn start(
        &mut self,
        start_functions: Vec<fn(&mut GameState, &Input) -> Result<GameAction, GameError>>,
        update_functions: Vec<fn(&mut GameState, &Input) -> Result<GameAction, GameError>>
    ) -> Result<(), Crash> {
        self.window.start(glfw::CursorMode::Normal);

        //run start functions
        for fun in &start_functions {
            let action = fun(&mut self.state, &self.window.input)?;
            self.handle_action(action);
        }

        while !self.window.should_close() {
            let input_action = self.window.process_input()?;
            self.handle_action(input_action);

            //run update functions
            for fun in &update_functions {
                let action = fun(&mut self.state, &self.window.input)?;
                self.handle_action(action);
            }

            let scene= self.get_scene();

            self.renderer.render(scene)?;
            self.window.swap_buffers()
        }
        Ok(())
    }

    fn handle_action(&mut self, action: GameAction) {
        match action {
            GameAction::Exit => self.quit(),
            GameAction::ChangeGameState(state) => self.state = state,
            GameAction::Resize(x, y) => {
                self.get_mut_scene()
                .get_mut_camera()
                .change_aspect_ratio(x as f32 / y as f32);
                self.renderer.resize((x, y))
            },
            GameAction::None => {},
        }
    }
    
    fn get_scene(&self) -> &dyn Scene {
        //bad syntax
        match &self.state {
            GameState::Menu(scene) => scene.as_ref(),
            GameState::Game(scene) => scene.as_ref(),
            GameState::Loading(scene) => scene.as_ref(),
        }
    }
    fn get_mut_scene(&mut self) -> &mut dyn Scene {
        match &mut self.state {
            GameState::Menu(scene) => scene.as_mut(),
            GameState::Game(scene) => scene.as_mut(),
            GameState::Loading(scene) => scene.as_mut(),
        }
    }

    fn quit(&mut self) {
        self.window.set_should_close(true);
    }
}