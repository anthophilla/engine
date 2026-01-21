use crate::{math::{self, Matrix4x4, Quaternion, Vector, Vector3}, vector};

#[derive(Debug)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub fov: f32,
    aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub perspective: Matrix4x4,
}
impl Camera {
    pub fn new(position: Vector3, rotation: Quaternion) -> Self {
        Self{
            position,
            rotation,
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
    pub fn set_world_position(&mut self, pos: Vector3) { self.position=pos; }
    pub fn set_rotation(&mut self, rot: Quaternion) { self.rotation=rot; }

    pub fn look_at(&self, target: Vector3) -> Matrix4x4 {
        let forward = target.normalize();
        let right = (forward.cross(vector!(0.0, 1.0, 0.0))).normalize();
        let up = right.cross(forward);

        Matrix4x4::from_arrays([
            [right.0[0],    right.0[1],    right.0[2],    right.dot(&self.position) ],
            [up.0[0],       up.0[1],       up.0[2],       up.dot(&self.position)     ],
            [-forward.0[0], -forward.0[1], -forward.0[2], -forward.dot(&self.position)],
            [0.0,           0.0,           0.0,           1.0]
        ])
    }
}