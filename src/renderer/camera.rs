use crate::math::{self, Matrix4x4, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub local_position: Vector3,
    pub fov: f32,
    aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub perspective: Matrix4x4,
}
impl Camera {
    pub fn new(local_position: Vector3) -> Self {
        Self{
            local_position,
            fov: 45.0,
            aspect_ratio: 1.0,
            near: 0.1,
            far: 100.0,
            perspective: math::perspective(45.0, 1.0, 0.1, 100.0)
        }
    }
    pub fn set_perspective(&mut self, fov: f32, aspect_ratio: f32, near: f32, far: f32) {
        self.fov = fov;
        self.aspect_ratio = aspect_ratio;
        self.near = near;
        self.far = far;
        self.perspective = math::perspective(fov, aspect_ratio, near, far);
    }

    //theese functions are boring and redundant but i think it keeps the ?api? clean
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.perspective = math::perspective(self.fov, aspect_ratio, self.near, self.far)
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.perspective = math::perspective(fov, self.aspect_ratio, self.near, self.far)
    }
    //pub fn translate(&mut self, offset: Vector3) { self.local_position = self.local_position+offset}
}