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

pub struct Renderer {
    vbo: [VertexBufferObject; 2],
    vao: [VertexArrayObject; 2],
    ebo: [ElementBufferObject; 2],
    shader_program: [u32; 2],
}
impl Renderer {
    pub fn new() -> Self {
        return Self{
            vbo: [VertexBufferObject(0), VertexBufferObject(0)], // ugly af
            vao: [VertexArrayObject(0), VertexArrayObject(0)],
            ebo: [ElementBufferObject(0), ElementBufferObject(0)],
            shader_program: [0, 0],
        };
    }
    pub fn init(&mut self, window: &mut glfw::Window) {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        self.vbo = [VertexBufferObject::new().unwrap(), VertexBufferObject::new().unwrap()];
        self.vao = [VertexArrayObject::new().unwrap(), VertexArrayObject::new().unwrap()];
        self.ebo = [ElementBufferObject::new().unwrap(), ElementBufferObject::new().unwrap()];
        
        self.viewport(WINDOW_SIZE_X.try_into().unwrap(), WINDOW_SIZE_Y.try_into().unwrap());
        
        //TODO: rewrite shaders as struct
        let vert_shader = self.load_shader(VERT_SHADER, gl::VERTEX_SHADER).unwrap();
        let frag_shader1 = self.load_shader(FRAG_SHADER1, gl::FRAGMENT_SHADER).unwrap();
        let frag_shader2 = self.load_shader(FRAG_SHADER2, gl::FRAGMENT_SHADER).unwrap();
        self.shader_program[0] = self.create_shader_program(vec![vert_shader, frag_shader1]).unwrap();
        self.shader_program[1] = self.create_shader_program(vec![vert_shader, frag_shader2]).unwrap();

        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }

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

        self.use_shader_program(self.shader_program[0]);
        self.vao[0].bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        self.use_shader_program(self.shader_program[1]);
        self.vao[1].bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }
        
        
        //self.ebo[1].bind();
        //unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());}
    }
    
    fn load_shader(&self, shader_src: &str, type_: gl::types::GLenum) -> Result<gl::types::GLuint, Error> {
            unsafe {
                let shader = gl::CreateShader(type_);
                if shader == 0 {return Err(Error::ShaderError("couldn't create shader"))}
                gl::ShaderSource(
                    shader,
                    1,
                    &(shader_src.as_bytes().as_ptr().cast()),
                    &(shader_src.len().try_into().unwrap()),
                );

                gl::CompileShader(shader);

                let mut success = 0;
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 { return Err(Error::ShaderError("couldn't compile shader")) }
                return Ok(shader);
            }
    }
    fn create_shader_program(&self, shaders: Vec<u32>) -> Result<u32, Error> {
        unsafe{
            let program = gl::CreateProgram();
            for shader in &shaders { gl::AttachShader(program, shader.clone()); }
            gl::LinkProgram(program);
            
            let mut success: i32 = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 { return Err(Error::ShaderError("couldn't create shader program :(")) }

            for shader in &shaders { gl::DeleteShader(shader.clone()); }
            return Ok(program)
        }
    }
    fn use_shader_program(&self, program: u32) {
        unsafe { gl::UseProgram(program); }
    }

    fn clear_color(&self, bg_color: (f32, f32, f32, f32)) {
        unsafe { gl::ClearColor(bg_color.0, bg_color.1, bg_color.2, bg_color.3); }
    }
    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    }

    fn viewport(&self, x: i32, y: i32) { unsafe { gl::Viewport(0, 0, x, y); } }
    pub fn resize(&self, x: i32, y: i32) { 
        println!("resize: {:?}", (x, y));
        self.viewport(x, y)
    }
}