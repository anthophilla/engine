pub mod game;
pub mod math;
pub mod renderer;

#[derive(Debug, Clone)]
pub enum Error{
    VAOGenError(&'static str),
    VBOGenError(&'static str),
    EBOGenError(&'static str),
    ShaderError(String),
    UniformError(&'static str),
    TextureError(String),
}