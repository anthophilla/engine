//use gl::types::*;
use crate::internals::game::Triangle;

// :(
const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;
const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

pub struct Renderer;
impl Renderer {
    pub fn new() -> Self {
        return Self;
    }
    pub fn init(&self, window: &mut glfw::Window) {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);
        //gl::ClearColor::load_with(|s| window.get_proc_address(s).unwrap() as *const _);
    }

    pub fn render(&self, triangle: Triangle) {
        Self.clear_color(crate::BACKGROUND_COLOR.as_tuple());

        let mut vao = 0;
        unsafe{ gl::GenVertexArrays(1, &mut vao); }
        assert_ne!(vao, 0);

        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        assert_ne!(vbo, 0);
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo) };

        let verts = triangle.as_array();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&verts) as isize,
                verts.as_ptr().cast(),
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of::<[f32; 3]>().try_into().unwrap(), 0 as *const _);
            gl::EnableVertexAttribArray(0);
        }
        let vert_shader = self.load_shader(VERT_SHADER, gl::VERTEX_SHADER);
        let frag_shader = self.load_shader(FRAG_SHADER, gl::FRAGMENT_SHADER);
        self.create_shader_program(vec![vert_shader, frag_shader]);

        self.clear();

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3);}
    }
    
    fn load_shader(&self, shader_src: &str, type_: gl::types::GLenum) -> gl::types::GLuint {
            unsafe {
                let shader = gl::CreateShader(type_);
                assert_ne!(shader, 0);
                gl::ShaderSource(
                    shader,
                    1,
                    &(shader_src.as_bytes().as_ptr().cast()),
                    &(shader_src.len().try_into().unwrap()),
                );

                gl::CompileShader(shader);

                let mut success = 0;
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 { panic!("Shader Compile Error!") }
                return shader;
            }
    }
    fn create_shader_program(&self, shaders: Vec<u32>) {
        unsafe{
            let program = gl::CreateProgram();
            for shader in &shaders { gl::AttachShader(program, shader.clone()); }
            gl::LinkProgram(program);
            
            let mut success: i32 = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 { panic!("Program Link Error!") }

            for shader in &shaders { gl::DeleteShader(shader.clone()); }
        }
    }

    fn clear_color(&self, bg_color: (f32, f32, f32, f32)) {
        unsafe { gl::ClearColor(bg_color.0, bg_color.1, bg_color.2, bg_color.3); }
    }
    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    }
}