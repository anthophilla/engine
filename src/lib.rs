pub mod renderer;
pub mod game;
pub mod math;
//pub use game::{Game, Scene, GameError};
//pub use renderer::WindowMode;

use game::GameError;

use std::fmt::Debug;
use renderer::{
    RenderError, WindowError
};

pub enum Crash {
    Unknown,
    RenderError(String),
    WindowError(String),
    GameError(String),
    InputError,
}
impl Debug for Crash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Crash::Unknown => write!(f, "encountered a unknown error!"),
            Crash::RenderError(msg) | Crash::WindowError(msg)
                => write!(f, "Renderer crashed: {msg}!"),
            Crash::GameError(msg) => write!(f, "Game logic crashed: {msg}"),
            Crash::InputError => write!(f, "encountered a problem while processing input")
        }
    }
}
impl From<RenderError> for Crash {
    fn from(value: RenderError) -> Self {
        Self::RenderError(
            match value {
                RenderError::InitError(msg) => format!("couldn't start the rendering process: {msg}"),
                RenderError::ShaderError(msg) => format!("shader error: {msg}"),
                RenderError::UniformError(msg) => format!("uniform error: {msg}"),
                RenderError::VBOError => "vbo error".to_string(),
                RenderError::VAOError => "vao error".to_string(),
                RenderError::EBOError => "ebo error".to_string(),
            }.to_string()
        )
    }
}
impl From<WindowError> for Crash {
    fn from(value: WindowError) -> Self {
        Self::WindowError(
            match value {
                WindowError::InitError => "couldn't initialize window",
                WindowError::CreateError => "couldn't create window"
            }.to_string()
        )
    }
}
impl From<GameError> for Crash {
    fn from(value: GameError) -> Self {
        Self::GameError(
            match value {
                GameError::Other(msg) => format!("custom game error: {}", msg)
            }.to_string()
        )
    }
}