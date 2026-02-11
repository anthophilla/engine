use engine::{
    Crash,
    game::{
        Game, GameAction, GameError, GameObject, GameState, Input, player::Player, scene::{MenuScene, Scene}, settings::{
            InputSettings,
            Settings, 
        }
    },
    math::{Vector, Vector3},
    renderer::{Camera, Texture, WindowMode, mesh::StaticMesh},
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
    fn get_position(&self) -> Vector3 { self.position }
    fn set_position(&mut self, pos: Vector3) { self.position = pos }
}

struct StartPlayer {
    camera: Camera,
    world_position: Vector3,
    speed: f32,
}
impl StartPlayer {
    fn new(world_position: Vector3, speed: f32) -> Self {
        let camera = Camera::new(
            world_position,
            90.0,
            1.0,
            100.0
        );
        Self { camera, world_position, speed }
    }
    fn get_speed(&self) -> f32 {
        self.speed
    }
}
impl Player for StartPlayer {
    fn get_camera(&self) -> &Camera { &self.camera }
    fn get_mut_camera(&mut self) -> &mut Camera { &mut self.camera }
    fn translate(&mut self, trans: Vector3) {
        self.world_position += trans;
        self.camera.set_position(self.world_position);
    }
}

struct StartScene {
    player: StartPlayer,
    cubes: Vec<Cube>,
}
impl StartScene {
    fn input(&mut self, input: &Input) -> Result<Vec<GameAction>, GameError> {
        let mut actions: Vec<GameAction> = vec![];
        
        if input.exit.0 == 1.0 { actions.push(GameAction::Exit) }

        let speed = self.player.get_speed();
        let delta = input.delta_time;
        let trans = vector!(
            (input.right.0 * speed * delta),
            0.0,
            -(input.forward.0 * speed * delta)
        );
        self.player.translate(trans);

        Ok(actions)
    }
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

    fn start(&mut self) -> Result<Vec<GameAction>, GameError> { Ok(vec![]) }
    fn update(&mut self, input: &Input) -> Result<Vec<GameAction>, GameError> {
        let mut actions: Vec<GameAction> = vec![];
        actions.extend(self.input(input)?);
        return Ok(actions)
    }
}
impl MenuScene for StartScene {}

//check if the game should quit
fn quit(_: &mut GameState, input: &Input) -> Result<GameAction, GameError> {
    if input.exit.0 == 1.0 { return Ok(GameAction::Exit) }

    Ok(GameAction::None)
}

fn first_scene() -> Result<GameState, GameError> {
    let cube1 = Cube::new(vector!(0.0, 1.0, -2.0), vec![
        Texture::from_file("src/textures/awesomeface.png")?,
        Texture::from_file("src/textures/container.jpg")?,
    ])?;
    let cube2 = Cube::new(vector!(0.0, -1.0, -2.0), vec![
        Texture::from_file("src/textures/awesomeface.png")?,
        Texture::from_file("src/textures/container.jpg")?,
    ])?;
    
    let scene = StartScene{
        player: StartPlayer::new(vector!(0.0, 0.0, 0.0), 0.5),
        cubes: vec![cube1, cube2]
    };

    Ok(GameState::Menu(
        Box::new(scene)
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

    game.start(first_scene()?)?;
    
    Ok(())
}