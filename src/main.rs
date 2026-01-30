use engine::{
    Crash,
    game::{
        Game,
        GameError,
        GameAction,
        Scene,
        InputSettings,
        Settings,
        Input
    },
    renderer::WindowMode,
};

//check if the game should quit
fn quit(scene: &mut Scene, input: &Input) -> Result<Option<GameAction>, GameError> {
    if input.exit.0 == 1.0 { return Ok(Some(GameAction::Exit)) }

    Ok(None)
}

fn main() -> Result<(), Crash> {
    let start_scene = Scene::default();
    let settings = Settings{
        game_title: "engine",
        window_size: (500, 500),
        window_mode: WindowMode::Windowed,

        input_settings: InputSettings::default()
    };
    let mut game = Game::init(start_scene, settings)?;

    let update_functions: Vec<fn(&mut Scene, &Input) -> Result<Option<engine::game::GameAction>, GameError>>
        = vec![quit];

    game.start(update_functions)?;
    
    Ok(())
}