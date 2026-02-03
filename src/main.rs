use engine::{
    Crash,
    game::{
        Game, GameAction, GameError, Input, InputSettings, Player, Scene, Settings
    },
    math::Vector,
    renderer::{Texture, WindowMode, mesh::StaticMesh},
    vector
};

//check if the game should quit
fn quit(_scene: &mut Scene, input: &Input) -> Result<GameAction, GameError> {
    if input.exit.0 == 1.0 { return Ok(GameAction::Exit) }

    Ok(GameAction::None)
}

fn first_scene(_: &mut Scene, _: &Input) -> Result<GameAction, GameError> {
    let textures = vec![
        Texture::from_file("src/textures/awesomeface.png")?,
        Texture::from_file("src/textures/container.jpg")?,
    ];
    
    let world = vec![
        StaticMesh::cube(
            (0.5, 0.5, 0.5),
            vector!(0.0, 0.0, -2.0),
            vector!(1.0, 1.0, 1.0, 1.0),
            textures
        ).map_err(|_| GameError::Other("TODO! mesh error".to_string()))?
    ];
    let scene = Scene::new(
        vec![],
        world,
        Player::default()
    );

    Ok(GameAction::LoadScene(scene))
}

fn main() -> Result<(), Crash> {  
    let settings = Settings{
        game_title: "engine",
        window_size: (500, 500),
        window_mode: WindowMode::Windowed,

        input_settings: InputSettings::default()
    };
    let mut game = Game::init(settings)?;

    let update_functions: Vec<fn(&mut Scene, &Input) -> Result<engine::game::GameAction, GameError>>
        = vec![quit];

    let start_functions: Vec<fn(&mut Scene, &Input) -> Result<GameAction, GameError>> = vec![first_scene];
    game.start(start_functions, update_functions)?;
    
    Ok(())
}