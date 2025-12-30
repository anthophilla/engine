use crate::internals::math::{Vector3};
use crate::internals::renderer::{Renderer}

struct Player {
    position: Vector3,
}

pub struct Game {
    player: Player,
    renderer: Renderer,
}