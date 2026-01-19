use engine::{
    game::{Game, Input, Player},
    math::Vector,
    vector
};

fn update(input: &Input, player: &mut Player, delta_time: f32) {
    if input.w { player.translate(vector!(0.0, 0.0, -1.0*player.speed*delta_time)); }
    if input.s { player.translate(vector!(0.0, 0.0, 1.0*player.speed*delta_time)); }
    if input.a { player.translate(vector!(-1.0*player.speed*delta_time, 0.0, 0.0)); }
    if input.d { player.translate(vector!(1.0*player.speed*delta_time, 0.0, 0.0)); }
    if input.space { player.translate(vector!(0.0, 1.0*player.speed*delta_time, 0.0)); }
    if input.shift { player.translate(vector!(0.0, -1.0*player.speed*delta_time, 0.0)); }
}

fn main() -> Result<(), engine::Error> {
    let mut player = Player::new();
    player.camera.set_fov(90.0);
    let mut game = Game::new(player);
    game.start(update)
}