use crate::{math::{Quaternion, Vector3}, renderer::Camera,};

// pub struct Player {
//     pub camera: Camera,
// }
// impl Player {
//     fn new() -> Self {
//         let camera = Camera::new(
//             vector!(0.0, 0.0, 0.0),
//             90.0,
//             1.0,
//             100.0,
//     );
//         Self { camera }
//     }
// }
// impl Default for Player {
//     fn default() -> Self {
//         Self::new()
//     }
// }

pub trait Player {
    fn get_camera(&self) -> &Camera;
    fn get_mut_camera(&mut self) -> &mut Camera;

    fn translate(&mut self, trans: Vector3);
    fn rotate(&mut self, rot: Quaternion);
}