use crate::{renderer::mesh::Mesh};

pub trait GameObject {
    fn get_mesh(&self) -> Option<Box<&dyn Mesh>>;
}