use crate::{math::{Mat4, Vector3, Vector}, vector};

pub struct Camera {
    world_position: Vector3,

    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,

    pub perspective: Mat4
}
impl Camera {
    pub fn new(
        world_position: Vector3,

        fov: f32,
        aspect_ratio: f32,
        far: f32,
    ) -> Self{
        Self {
            world_position,
            fov,
            aspect_ratio,
            near: 0.1,
            far,
            perspective: crate::math::perspective(fov, aspect_ratio, 0.1, far),
        }
    }

    //two useless functions but ill keep them
    pub fn change_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.recalculate_perspective();
    }
    pub fn change_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.recalculate_perspective();
    }

    fn recalculate_perspective(&mut self) {
        self.perspective = crate::math::perspective(self.fov, self.aspect_ratio, self.near, self.far)
    }

    //pub fn look_at()
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(vector!(0.0, 0.0, 0.0), 90.0, 1.0, 100.0)
    }
}