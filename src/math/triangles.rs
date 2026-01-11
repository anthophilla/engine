use crate::math::Vector3;

pub struct Triangle(Vector3, Vector3, Vector3);
impl Triangle {
    pub const fn new(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self(x,y,z)
    }
    pub const fn as_array(&self) -> [[f32;3];3] {
        [
            self.0.as_array(),
            self.1.as_array(),
            self.2.as_array(),
        ]
    }
}