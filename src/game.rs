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
        &mut self, start_state: GameState
    ) -> Result<(), Crash> {
        self.window.start(glfw::CursorMode::Normal);
        self.change_state(start_state)?;
        
        //execute start function
        for action in self.get_mut_scene().start()? {
            self.handle_action(action)?;
        }

        while !self.window.should_close() {
            let input_action = self.window.process_input()?;
            self.handle_action(input_action)?;

            let input = &mut self.window.input;
            
            //bro....
            //execute update functions
            let actions = {
                let scene: &mut dyn Scene = match &mut self.state {
                    GameState::Menu(scene) => scene.as_mut(),
                    GameState::Game(scene) => scene.as_mut(),
                    GameState::Loading(scene) => scene.as_mut(),
                };
                scene.update(&input)?
            };

            for action in actions {
                self.handle_action(action)?;
            }
            
            self.renderer.render(self.get_scene())?;
            self.window.swap_buffers()
        }
        Ok(())
    }

    fn handle_action(&mut self, action: GameAction) -> Result<(), Crash> {
        match action {
            GameAction::Exit => self.quit(),
            GameAction::ChangeGameState(state) => {
                self.state = state;
                for action in self.get_mut_scene().start()? {
                    self.handle_action(action)?;
                }
            },
            GameAction::Resize(x, y) => {
                self.get_mut_scene()
                .get_mut_camera()
                .change_aspect_ratio(x as f32 / y as f32);
                self.renderer.resize((x, y))
            },
            GameAction::None => {},
        }
        Ok(())
    }

    fn change_state(&mut self, state: GameState) -> Result<(), Crash> {
        self.state = state;
        Ok(())
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