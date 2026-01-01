use gl::types::*;
use crate::internals::math::Triangle;
use crate::{WINDOW_SIZE_X, WINDOW_SIZE_Y};
use crate::internals::Error;

// :(
const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;
const FRAG_SHADER1: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;
const FRAG_SHADER2: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(0.0, 1.0, 0.0, 1.0);
  }
"#;

struct VertexArrayObject(GLuint);
impl VertexArrayObject {
    pub fn new() -> Result<Self, Error> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        if vao!=0 {
            return Ok(VertexArrayObject(vao))
        }
        else{ Err(Error::VAOGenError("cannot create vao object")) }
    }

    pub fn bind(&self) {
        unsafe {gl::BindVertexArray(self.0);}
    }
    pub fn unbind(&self) {
        unsafe {gl::BindVertexArray(0);}
    }
}

struct VertexBufferObject(GLuint);
impl VertexBufferObject {
    fn new() -> Result<Self, Error> {
        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        if vbo != 0 {return Ok(Self(vbo))}
        else {return Err(Error::VBOGenError("cannot create vbo"))}
    }
    pub fn bind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, self.0);}
    }
    pub fn _unbind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0);}
    }
    pub fn buffer_vertices(&self, vertices: &[f32], usage: GLenum) {
        //dbg!(&vertices);
        unsafe{
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(vertices) as isize,
                vertices.as_ptr().cast(),
                usage,
            );
        }
    }
}

struct ElementBufferObject(GLuint);
impl ElementBufferObject {
    fn new() -> Result<Self, Error> {
        let mut ebo = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }
        if ebo != 0 {return Ok(Self(ebo))}
        else {return Err(Error::EBOGenError("cannot create ebo"))}
    }
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.0); }
    }
    pub fn _unbind(&self) {
        unsafe {gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);}
    }
    pub fn buffer_elements(&self, vertices: Vec<u32>, usage: GLenum) {
        
        unsafe{
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                usage,
            );
        }
    }
}

struct Shader(u32);
impl Shader {
    fn create(source: &'static str, shader_type: u32) -> Result<Self, Error> {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            if shader == 0 {return Err(Error::ShaderError("couldn't create shader".to_string()))}
            gl::ShaderSource(
                shader,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );

            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut info_buffer: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(shader, 1024, &mut log_len, info_buffer.as_mut_ptr().cast());
                //info_buffer.set_len(log_len.try_into().unwrap());
                let msg = format!("Shader compile error: {}", String::from_utf8_lossy(&info_buffer));
                return Err(Error::ShaderError(msg))
            }
            return Ok(Self(shader));
        }
    }
}
struct ShaderProgram {
    program: u32,
    vertex_shader: Shader,
    frag_shader:   Shader,
}
impl ShaderProgram {
    fn create(vertex_shader: Shader, frag_shader: Shader) -> Result<Self, Error> {
        unsafe{
            let program = gl::CreateProgram();
            
            gl::AttachShader(program, vertex_shader.0.clone());
            gl::AttachShader(program, frag_shader.0.clone());

            gl::LinkProgram(program);
            
            let mut success: i32 = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 { 
                let mut info_buffer: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(program, 1024, &mut log_len, info_buffer.as_mut_ptr().cast());
                //info_buffer.set_len(log_len.try_into().unwrap());
                let msg = format!("Shader program link error: {}", String::from_utf8_lossy(&info_buffer));
                return Err(Error::ShaderError(msg))
            }

            gl::DeleteShader(vertex_shader.0.clone());
            gl::DeleteShader(frag_shader.0.clone());
            return Ok(ShaderProgram { program, vertex_shader, frag_shader })
        }
    }
    fn use_program(&self) {
        unsafe { gl::UseProgram(self.program); }
    }
}

pub struct Renderer {
    vbo: [VertexBufferObject; 2],
    vao: [VertexArrayObject; 2],
    ebo: [ElementBufferObject; 2],
    shader_program: [ShaderProgram; 2],
}
impl Renderer {
    pub fn init(window: &mut glfw::Window) -> Self {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        let vbo = [VertexBufferObject::new().unwrap(), VertexBufferObject::new().unwrap()];
        let vao = [VertexArrayObject::new().unwrap(), VertexArrayObject::new().unwrap()];
        let ebo = [ElementBufferObject::new().unwrap(), ElementBufferObject::new().unwrap()];
        
        Self::viewport(WINDOW_SIZE_X.try_into().unwrap(), WINDOW_SIZE_Y.try_into().unwrap());
        
        //TODO: rewrite shaders as struct
        let vert_shader1 = Shader::create(VERT_SHADER, gl::VERTEX_SHADER).unwrap();
        let vert_shader2 = Shader::create(VERT_SHADER, gl::VERTEX_SHADER).unwrap();
        let frag_shader1 = Shader::create(FRAG_SHADER1, gl::FRAGMENT_SHADER).unwrap();
        let frag_shader2 = Shader::create(FRAG_SHADER2, gl::FRAGMENT_SHADER).unwrap();
        let shader_program= [
            ShaderProgram::create(vert_shader1, frag_shader1).unwrap(),
            ShaderProgram::create(vert_shader2, frag_shader2).unwrap(),
        ];

        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }

        return Self{
            vbo,
            vao,
            ebo,
            shader_program,
        };
    }
    
    pub fn render(&self, triangles: Vec<Triangle>) {
        for (i, triangle) in triangles.iter().enumerate() {
            //let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
            let mut verts: Vec<f32> = vec![];
            for i in triangle.as_array() {
                for x in i { verts.push(x); }
            }
            
            self.vao[i].bind();
            self.vbo[i].bind();
            self.vbo[i].buffer_vertices(&verts[..], gl::STATIC_DRAW);
    
            //explain to gl how to read supplied vertices and save it to vao
            unsafe {
                //size_of::<[f32; 3]>().try_into().unwrap()
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const _);
                gl::EnableVertexAttribArray(0);
            }
            //self.ebo[i].bind();
            //self.ebo[i].buffer_elements(indices, gl::STATIC_DRAW);
        }

        self.clear_color(crate::BACKGROUND_COLOR.as_tuple());
        self.clear();

        self.shader_program[0].use_program();
        self.vao[0].bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        self.shader_program[1].use_program();
        self.vao[1].bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }
        
        
        //self.ebo[1].bind();
        //unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());}
    }

    fn clear_color(&self, bg_color: (f32, f32, f32, f32)) {
        unsafe { gl::ClearColor(bg_color.0, bg_color.1, bg_color.2, bg_color.3); }
    }
    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    }

    fn viewport(x: i32, y: i32) { unsafe { gl::Viewport(0, 0, x, y); } }
    pub fn resize(&self, x: i32, y: i32) { 
        println!("resize: {:?}", (x, y));
        Self::viewport(x, y)
    }
}