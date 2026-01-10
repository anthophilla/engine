use engine::game::Game;

fn main() -> Result<(), engine::Error> {
    let mut game = Game::new();
    game.start()
}