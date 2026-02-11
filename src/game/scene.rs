use crate::{game::GameObject, renderer::{Camera, mesh::{Mesh, StaticMesh}}};

pub trait Scene {
    //fn get_static_meshes(&self) -> Vec<StaticMesh>;
    fn get_game_objects(&self) -> Vec<Box<&(dyn GameObject + 'static)>>;
    // fn get_meshes(&self) -> Vec<&Box<dyn Mesh>> {
    //     let mut result: Vec<&Box<dyn Mesh>> = vec![];
    //     for obj in self.get_game_objects() {
    //         match obj.get_mesh() {
    //             Some(mesh) => result.push(&mesh),
    //             None => {},
    //         };
    //     };
    //     return result
    // }
    // fn meshes(&self) -> Box<dyn Iterator<Item = &dyn Mesh> + '_> {
    //     Box::new(
    //         self.get_game_objects()
    //             .iter()
    //             .filter_map(|obj| obj.get_mesh()).into_iter()
    //     )
    // }
    fn for_each_mesh(&self, f: &mut dyn FnMut(&dyn Mesh)) {
        for obj in self.get_game_objects() {
            if let Some(mesh) = obj.get_mesh() {
                f(*mesh)
            }
        }
    }
    //fn get_meshes(&self) -> Vec<Box<dyn Mesh>>;
    fn get_current_camera(&self) -> &Camera;
    fn get_mut_camera(&mut self) -> &mut Camera;
}

pub trait MenuScene: Scene {}
pub trait GameScene: Scene {}
pub trait LoadingScene: Scene {}

pub struct EmptyScene(Camera);
impl EmptyScene {
    pub fn new() -> Self {
        Self(Camera::default())
    }
}
impl Scene for EmptyScene {
    fn get_current_camera(&self) -> &Camera {
        return &self.0
    }
    fn get_mut_camera(&mut self) -> &mut Camera {
        return &mut self.0
    }
    fn get_game_objects(&self) -> Vec<Box<&(dyn GameObject + 'static)>> {
        vec![]
    }
    //fn get_static_meshes(&self) -> Vec<St> { vec![] }
}
impl LoadingScene for EmptyScene {}