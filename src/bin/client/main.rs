mod internals;

use internals::{
    game::Game,
    math::Vector4
};

pub static GAME_NAME: &str = "enigne";
pub const WINDOW_SIZE_X: u32 = 300;
pub const WINDOW_SIZE_Y: u32 = 300;
pub static BACKGROUND_COLOR: Vector4 = Vector4::new(0.5, 0.3, 0.3, 1.0);

fn main() -> Result<(), ()> {
    let mut game = Game::new();
    return game.start()
}
