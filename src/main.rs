use engine::{
    Crash,
    game::{
        Game, GameAction, GameError, GameObject, GameState, Input, InputSettings, Player, Scene, Settings
    },
    math::{Vector, Vector3},
    renderer::{Texture, WindowMode, mesh::StaticMesh},
    vector
};

struct Cube {
    mesh: StaticMesh,
    position: Vector3,
}
impl Cube {
    fn new(position: Vector3, textures: Vec<Texture>) -> Result<Self, GameError> {
        let mesh = StaticMesh::cube(
            (0.5, 0.5, 0.5),
            position,
            vector!(1.0, 1.0, 1.0, 1.0),
            textures
        ).map_err(|_| GameError::Other("TODO! mesh error".to_string()))?;

        Ok(Self {
            mesh,
            position
        })
    }
}
impl GameObject for Cube {
    fn get_mesh(&self) -> Option<Box<&dyn engine::renderer::mesh::Mesh>> {
        Some(Box::new(&self.mesh))
    }
}

struct StartScene {
    player: Player,
    cubes: Vec<Cube>
}
impl Scene for StartScene {
    fn get_current_camera(&self) -> &engine::renderer::Camera {
        &self.player.camera
    }
    fn get_mut_camera(&mut self) -> &mut engine::renderer::Camera {
        &mut self.player.camera
    }
    fn get_game_objects(&self) -> Vec<Box<&(dyn GameObject + 'static)>> {
        // self.cubes
        //     .iter()
        //     .map(|cube| Box::new(cube))
        //     .collect()
        let mut result: Vec<Box<&(dyn GameObject + 'static)>> = vec![];
        for cube in &self.cubes {
            result.push(Box::new(cube))
        }
        return result
    }
}

//check if the game should quit
fn quit(_: &mut GameState, input: &Input) -> Result<GameAction, GameError> {
    if input.exit.0 == 1.0 { return Ok(GameAction::Exit) }

    Ok(GameAction::None)
}

fn first_scene(_: &mut GameState, _: &Input) -> Result<GameAction, GameError> {
    let textures = vec![
        Texture::from_file("src/textures/awesomeface.png")?,
        Texture::from_file("src/textures/container.jpg")?,
    ];
    let cube = Cube::new(vector!(0.0, 0.0, -2.0), textures)?;
    
    let scene = StartScene{
        player: Player::default(),
        cubes: vec![cube]
    };

    Ok(GameAction::ChangeGameState(
        GameState::Game(
            Box::new(scene)
        )
    ))
}

fn main() -> Result<(), Crash> {  
    let settings = Settings{
        game_title: "engine",
        window_size: (500, 500),
        window_mode: WindowMode::Windowed,

        input_settings: InputSettings::default()
    };
    let mut game = Game::init(settings)?;

    let update_functions: Vec<fn(&mut GameState, &Input) -> Result<engine::game::GameAction, GameError>>
        = vec![quit];

    let start_functions: Vec<fn(&mut GameState, &Input) -> Result<GameAction, GameError>> = vec![first_scene];
    game.start(start_functions, update_functions)?;
    
    Ok(())
}