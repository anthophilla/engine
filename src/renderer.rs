pub mod camera;
pub mod window;
pub mod mesh;
mod buffers;
mod uniform;
mod shaders;
mod texture;

pub use camera::Camera;
pub use window::{Window, WindowError, WindowMode};
pub use texture::Texture;

use std::mem::offset_of;

use mesh::{StaticMesh, Mesh};
use uniform::Uniform;
use shaders::{Shader, ShaderProgram, ShaderType};

use crate::{
    game::{
        scene::Scene,
        settings::Settings
    },
    math::{Color, Mat4, Vector, Vector3},
    vector
};

const BACKGROUND_COLOR: Color = vector!(0.5, 0.0, 0.0, 1.0);

pub enum RenderError {
    InitError(String),
    VAOError,
    VBOError,
    EBOError,
    ShaderError(String),
    UniformError(String),
    TextureError(String),
}

#[derive(Debug)]
#[repr(C)]
pub struct Vertex{
    position: [f32; 3],
    color:    [f32; 4],
    uv:       [f32; 2],
}
impl Vertex {
    pub fn from_vectors(position: Vector3, color: Color, uv: Vector<2>) -> Self {
        Self{
            position: position.into(),
            color: color.into(),
            uv: uv.into(),
        }
    }
    pub fn set_attrib_pointers() { unsafe{
        let stride = size_of::<Vertex>() as i32;

        //vertices
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Self, position) as *const _);
        gl::EnableVertexAttribArray(0);
        //color
        gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, offset_of!(Self, color) as *const _);
        gl::EnableVertexAttribArray(1);
        //texture
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Self, uv) as *const _);
        gl::EnableVertexAttribArray(2);
    }}
}

pub struct Renderer {
    shader_programs: Vec<ShaderProgram>,
}

impl Renderer {
    pub fn init(window: &mut glfw::Window, settings: &Settings) -> Result<Self, RenderError> {
        //bad practise unwrap
        gl::load_with(
            |s| window.get_proc_address(s).unwrap() as *const _
        );

        Self::set_viewport(settings.window_size.0 as i32, settings.window_size.1 as i32);
        
        let vertex_shader = Shader::from_file("src/shaders/shader.vert", ShaderType::Vertex)?;
        let frag_shader   = Shader::from_file("src/shaders/shader.frag", ShaderType::Fragment)?;

        let program = ShaderProgram::create(vertex_shader, frag_shader)?;

        unsafe { gl::Enable(gl::DEPTH_TEST); }
        Self::set_texture_params();

        Ok(Self {
            shader_programs: vec![program]
        })
    }

    pub fn render(&self, scene: &dyn Scene) -> Result<(), RenderError> {
        let camera = scene.get_current_camera();
        
        self.clear_color(BACKGROUND_COLOR);
        self.clear();

        let program = &self.shader_programs[0];
        program.use_program();

        program.texture0.seti1(0);
        program.texture1.seti1(1);

        program.perspective.setmat4(camera.perspective);
        //program.perspective.setmat4(Mat4::IDENTITY);
        program.view.setmat4(Mat4::IDENTITY);

        let mut mesh_func = |mesh: &dyn Mesh| mesh.draw(&program.model_transform, &program.model_rotation);
        scene.for_each_mesh(&mut mesh_func);

        Ok(())
    }

    pub fn resize(&self, (x, y): (u32, u32)) { Self::set_viewport(x as i32, y as i32); }

    fn set_viewport(x: i32, y: i32) { unsafe { gl::Viewport(0, 0, x, y); } }
    fn clear_color(&self, color: Color) {unsafe {
        gl::ClearColor(color.0[0], color.0[1], color.0[2], color.0[3]);
    }}
    fn clear(&self) {unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }}

    fn set_texture_params() {unsafe {
        //i think the unwraps here arent that bad
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());
    }}
}