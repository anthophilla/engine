use crate::{
    math::{Quaternion, Vector3},
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
    fn set_rotation(&mut self, rotation: Quaternion);
    fn get_rotation(&self) -> Quaternion;
    ///current_rotation * offset
    fn rotate(&mut self, offset: Quaternion) {
        self.set_rotation(self.get_rotation()*offset);
    }
}