use std::collections::HashMap;
use gl::{types::*};
use crate::internals::math::Triangle;
use crate::{WINDOW_SIZE_X, WINDOW_SIZE_Y};
use crate::internals::Error;

// :(
const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 aPos;
  layout (location = 1) in vec4 aColor;
  //vec4 aColor = vec4(0.0, 0.0, 0.0, 1.0);
  out vec4 vertexColor;

  void main() {
    gl_Position = vec4(aPos, 1.0);
    vertexColor = aColor;
  }
"#;
const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 FragColor;
  in vec4 vertexColor;
  //uniform vec4 customColor;
  void main() {
    FragColor = vertexColor;
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
    pub fn buffer(&self, vertices: &[([f32; 3], [f32; 4])], usage: GLenum) {
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
    fn create(source: &[u8], shader_type: u32) -> Result<Self, Error> {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            if shader == 0 {return Err(Error::ShaderError("couldn't create shader".to_string()))}
            gl::ShaderSource(
                shader,
                1,
                &(source.as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );

            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut info_buffer: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(shader, 1024, &mut log_len, info_buffer.as_mut_ptr().cast());
                info_buffer.set_len(log_len.try_into().unwrap());
                let msg = format!("Shader compile error: {}", String::from_utf8_lossy(&info_buffer));
                return Err(Error::ShaderError(msg))
            }
            return Ok(Self(shader));
        }
    }
    pub fn from_file(path: &'static str, shader_type: u32) -> Result<Self, Error> {
        let source = match std::fs::read(path) {
            Ok(d) => d,
            Err(_) => return Err(Error::ShaderError(format!("couldn't find: {}", path).to_string()))
        };
        Self::create(&source, shader_type)
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
                info_buffer.set_len(log_len.try_into().unwrap());
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

trait Uniform {
    fn new(location: i32) -> Self where Self: Sized;
    fn from_name(name: &'static str, shader_program: &ShaderProgram) -> Result<Self, Error> where Self: Sized {
        let uniform = unsafe { gl::GetUniformLocation(shader_program.program, name.as_ptr().cast()) };
        //REMEMBER TO ADD A NULL BYTE to name ('\0')
        if uniform==-1 {return Err(Error::UniformError(name));}
        return Ok(Self::new(uniform))
    }
}
struct UniformF<const N: usize> { location: i32 }
impl<const N: usize> Uniform for UniformF<N> {
    fn new(location: i32) -> Self { Self{location} }
}

impl UniformF<1> {
    fn set(&self, x: f32) { unsafe { gl::Uniform1f(self.location, x); } }
}
impl UniformF<2> {
    fn set(&self, x: f32, y: f32) { unsafe { gl::Uniform2f(self.location, x, y); } }
}
impl UniformF<3> {
    fn set(&self, x: f32, y: f32, z: f32) { unsafe { gl::Uniform3f(self.location, x, y, z); } }
}
impl UniformF<4> {
    fn set(&self, x: f32, y: f32, z: f32, w: f32) { unsafe { gl::Uniform4f(self.location, x, y, z, w); } }
}

// this is sadly the only way to do this?
enum AnyUniform {
    F1(UniformF<1>),
    F2(UniformF<2>),
    F3(UniformF<3>),
    F4(UniformF<4>),
}


pub struct Renderer {
    vbo: [VertexBufferObject; 2],
    vao: [VertexArrayObject; 2],
    ebo: [ElementBufferObject; 2],
    shader_program: Vec<ShaderProgram>,
    uniforms: HashMap<&'static str, AnyUniform>,
}
impl Renderer {
    pub fn init(window: &mut glfw::Window) -> Self {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        let vbo = [VertexBufferObject::new().unwrap(), VertexBufferObject::new().unwrap()];
        let vao = [VertexArrayObject::new().unwrap(), VertexArrayObject::new().unwrap()];
        let ebo = [ElementBufferObject::new().unwrap(), ElementBufferObject::new().unwrap()];
        
        Self::viewport(WINDOW_SIZE_X.try_into().unwrap(), WINDOW_SIZE_Y.try_into().unwrap());
        
        //TODO: rewrite shaders as struct
        let vert_shader1 = Shader::from_file("src/bin/client/shaders/shader.vert", gl::VERTEX_SHADER).unwrap();
        let frag_shader1 = Shader::from_file("src/bin/client/shaders/shader.frag", gl::FRAGMENT_SHADER).unwrap();
        let shader_program= vec![
            ShaderProgram::create(vert_shader1, frag_shader1).unwrap()];
        
        let uniforms = HashMap::from([
            //("offset", AnyUniform::F3(UniformF::from_name("offset\0", &shader_program[0]).unwrap())),
        ]);

        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        return Self{
            vbo,
            vao,
            ebo,
            shader_program,
            uniforms,
        };
    }
    
    pub fn render(&self, triangles: Vec<Triangle>) -> Result<(), Error> {
        for (i, triangle) in triangles.iter().enumerate() {
            //let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
            let mut verts: Vec<([f32; 3], [f32; 4])> = vec![];
            for i in triangle.as_array() {
                verts.push((i, [1.0, 0.0, 0.0, 1.0]));
            }
            dbg!(&verts);
            
            self.vao[i].bind();
            self.vbo[i].bind();
            self.vbo[i].buffer(&verts[..], gl::STATIC_DRAW);
    
            //explain to gl how to read supplied vertices and save it to vao
            unsafe {
                //vertices
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of::<([f32; 3], [f32; 4])>() as i32, 0 as *const _);
                gl::EnableVertexAttribArray(0);
                //color
                gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, size_of::<([f32; 3], [f32; 4])>() as i32, size_of::<[f32; 3]>() as *const _);
                gl::EnableVertexAttribArray(1);
            }
            //self.ebo[i].bind();
            //self.ebo[i].buffer_elements(indices, gl::STATIC_DRAW);
        }

        self.clear_color(crate::BACKGROUND_COLOR.as_tuple());
        self.clear();

        //let AnyUniform::F3(offset) = self.uniforms.get("offset").unwrap()
        //    else { return Err(Error::UniformError("customColor")); };
        //offset.set(0.0, 1.0, 0.0);

        self.shader_program[0].use_program();
        self.vao[0].bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        //offset.set(-1.0, 0.0, 0.0);

        self.shader_program[0].use_program();
        self.vao[1].bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }
        
        //self.ebo[1].bind();
        //unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());}
        return Ok(()) //TODO: return deltatime
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