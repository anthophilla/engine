pub mod game;
pub mod math;
pub mod renderer;

use math::{Color};

#[derive(Debug, Clone)]
pub enum Error{
    VAOGenError(&'static str),
    VBOGenError(&'static str),
    EBOGenError(&'static str),
    ShaderError(String),
    UniformError(&'static str),
    TextureError(String),
}

pub static GAME_NAME: &str = "enigne";
pub const WINDOW_SIZE_X: u32 = 300;
pub const WINDOW_SIZE_Y: u32 = 300;
pub static BACKGROUND_COLOR: Color = Color::new(0.5, 0.3, 0.3, 1.0);