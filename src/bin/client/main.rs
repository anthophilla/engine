mod internals;

use internals::game::Game;

pub static GAME_NAME: &str = "enigne";
pub static WINDOW_SIZE_X: u32 = 300;
pub static WINDOW_SIZE_Y: u32 = 300;

fn main() -> Result<(), ()> {
    let game = Game::new();
    return game.start()
}
