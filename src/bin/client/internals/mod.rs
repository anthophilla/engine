pub mod game;
pub mod math;
pub mod renderer;

#[derive(Debug, Clone)]
enum Error{
    VAOGenError(&'static str),
    VBOGenError(&'static str),
    EBOGenError(&'static str),
    ShaderError(&'static str),
}