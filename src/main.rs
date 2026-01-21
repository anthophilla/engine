use engine::{
    game::{Game, Input, Player},
    math::{Vector, Quaternion},
    vector
};

fn update(input: &Input, player: &mut Player, delta_time: f32) {
    let speed = player.speed*delta_time;

    player.rotate(
        Quaternion::from_radian_vect(input.cursor_diff.0 as f32 * delta_time, vector!(0.0, 1.0, 0.0))
        * Quaternion::from_radian_vect(-input.cursor_diff.1 as f32 * delta_time, vector!(1.0, 0.0, 0.0))
    );
    if input.w {
        player.translate(vector!(0.0, 0.0, 1.0*speed).rotate(player.rotation));
    }
    if input.s {
        player.translate(vector!(0.0, 0.0, -1.0*speed).rotate(player.rotation));}
    if input.a { player.translate(vector!(1.0*speed, 0.0, 0.0).rotate(player.rotation)); }
    if input.d { player.translate(vector!(-1.0*speed, 0.0, 0.0).rotate(player.rotation)); }
    if input.space { player.translate(vector!(0.0, 1.0*speed, 0.0).rotate(player.rotation)); }
    if input.shift { player.translate(vector!(0.0, -1.0*speed, 0.0).rotate(player.rotation)); }
}

fn main() -> Result<(), engine::Error> {
    let mut player = Player::new(
        vector!(0.0, 0.0, 0.0),
        Quaternion::from_angle_vect(95.0, vector!(0.0, 1.0, 0.0)),
        2.0,
        0.5,
    );
    player.camera.set_fov(90.0);
    let mut game = Game::new(player);
    game.start(update)
}