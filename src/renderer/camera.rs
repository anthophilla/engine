use crate::{game::GameObject, math::{Mat3, Mat4, Quaternion, Vector, Vector3}, vector};

pub struct Camera {
    world_position: Vector3,
    rotation: Quaternion,

    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,

    pub perspective: Mat4
}
impl Camera {
    pub fn new(
        world_position: Vector3,
        rotation: Quaternion,
        fov: f32,
        aspect_ratio: f32,
        far: f32,
    ) -> Self{
        Self {
            world_position,
            rotation,
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

    ///returns lookat matrix in worldspace
    pub fn look_at(&self) -> Mat4 {
        let start = self.world_position * vector!(-1.0, 1.0, -1.0);
        
        let forward = (Mat3::from(self.rotation) * -Vector3::FRONT).normalized();
        let right = (forward.crossed(&Vector3::UP)).normalized();
        let up = right.crossed(&forward);

        Mat4::from_arrays([
            [right[0],    right[1],    right[2],    -right.dot(&start)  ],
            [up[0],       up[1],       up[2],       -up.dot(&start)     ],
            [-forward[0], -forward[1], -forward[2], forward.dot(&start)],
            [0.0,           0.0,           0.0,           1.0]
        ])
    }
}

impl GameObject for Camera {
    fn get_mesh(&self) -> Option<Box<&dyn super::mesh::Mesh>> { None }
    fn get_position(&self) -> Vector3 { self.world_position }
    fn set_position(&mut self, pos: Vector3) { self.world_position = pos }
    fn change_position(&mut self, offset: Vector3) { self.world_position += offset }
    fn get_rotation(&self) -> Quaternion { self.rotation }
    fn set_rotation(&mut self, rotation: Quaternion) { self.rotation = rotation; }
    fn rotate(&mut self, offset: Quaternion) {
        self.rotation = self.rotation*offset;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            vector!(0.0, 0.0, 0.0),
            Quaternion::IDENTITY,
            90.0,
            1.0,
            100.0
        )
    }
}