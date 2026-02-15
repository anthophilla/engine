use engine::{
    Crash,
    game::{
        Game, GameAction, GameError, GameObject, GameState, Input, player::Player, scene::{MenuScene, Scene}, settings::{
            InputSettings,
            Settings, 
        }
    },
    math::{Mat3, Quaternion, Vector, Vector3},
    renderer::{Camera, Texture, WindowMode, mesh::StaticMesh},
    vector
};

struct Cube {
    mesh: StaticMesh,
    position: Vector3,
    rotation: Quaternion,
}
impl Cube {
    fn new(position: Vector3, rotation: Quaternion, textures: Vec<Texture>) -> Result<Self, GameError> {
        let mesh = StaticMesh::cube(
            (0.5, 0.5, 0.5),
            position,
            rotation,
            vector!(1.0, 1.0, 1.0, 1.0),
            textures
        ).map_err(|_| GameError::Other("TODO! mesh error".to_string()))?;

        Ok(Self {
            mesh,
            position,
            rotation,
        })
    }
}
impl GameObject for Cube {
    fn get_mesh(&self) -> Option<Box<&dyn engine::renderer::mesh::Mesh>> {
        Some(Box::new(&self.mesh))
    }
    fn get_position(&self) -> Vector3 { self.position }
    fn set_position(&mut self, pos: Vector3) {
        self.position = pos;
        self.mesh.set_position(self.position);
    }
    fn get_rotation(&self) -> engine::math::Quaternion { self.rotation }
    fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation;
        self.mesh.set_rotation(self.rotation);
    }
}

struct StartPlayer {
    camera: Camera,
    rotation: Quaternion,
    world_position: Vector3,
    speed: f32,
}
impl StartPlayer {
    fn new(world_position: Vector3, speed: f32) -> Self {
        let camera = Camera::new(
            world_position,
            Quaternion::from_angle_vect(0.0, vector!(1.0, 1.0, 1.0)),
            90.0,
            1.0,
            100.0
        );
        Self {
            camera,
            rotation: Quaternion::IDENTITY,
            world_position,
            speed
        }
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
    fn rotate(&mut self, rot: Quaternion) {
        self.rotation = self.rotation*rot;
        self.camera.set_rotation(self.rotation);
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

        self.cubes[0].rotate(Quaternion::from_angle_vect(5.0*input.delta_time, vector!(0.5, 0.5, 0.5)));

        return Ok(actions)
    }
}
impl MenuScene for StartScene {}

fn first_scene() -> Result<GameState, GameError> {
    let cube1 = Cube::new(vector!(0.0, 1.0, -2.0), Quaternion::IDENTITY,
    vec![
        Texture::from_file("src/textures/awesomeface.png")?,
        Texture::from_file("src/textures/container.jpg")?,
    ])?;
    let cube2 = Cube::new(vector!(0.0, -1.0, -2.0), Quaternion::IDENTITY,vec![
        Texture::from_file("src/textures/awesomeface.png")?,
        Texture::from_file("src/textures/container.jpg")?,
    ])?;
    
    let scene = StartScene{
        player: StartPlayer::new(vector!(0.0, 0.0, 0.0), 1.5),
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