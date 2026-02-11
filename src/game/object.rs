use crate::{
    math::Vector3,
    renderer::mesh::Mesh
};

pub trait GameObject {
    fn get_mesh(&self) -> Option<Box<&dyn Mesh>>;
    fn get_position(&self) -> Vector3;
    fn set_position(&mut self, pos: Vector3);
    ///change position by offset
    fn change_position(&mut self, offset: Vector3) {
        self.set_position(self.get_position()+offset); //is this slow???
    }
}