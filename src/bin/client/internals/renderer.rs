use std::collections::HashMap;
use std::mem::offset_of;
use std::io::Cursor;
use gl::{types::*};
use image::ImageReader;
use crate::internals::math::Triangle;
use crate::{WINDOW_SIZE_X, WINDOW_SIZE_Y};
use crate::internals::Error;

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
    pub fn buffer(&self, vertices: &[Vertex], usage: GLenum) {
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
    pub fn buffer_elements(&self, indices: Vec<[i32; 3]>, usage: GLenum) {
        
        unsafe{
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size_of_val(&indices) as isize,
                indices.as_ptr().cast(),
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
    uniforms: HashMap<&'static str, AnyUniform>
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
            return Ok(ShaderProgram { program, vertex_shader, frag_shader, uniforms: HashMap::new() })
        }
    }
    fn use_program(&self) {
        unsafe { gl::UseProgram(self.program); }
    }
    fn add_uniform(&mut self, t: UniformType, name: &'static str) -> Result<AnyUniform, Error> {
        self.uniforms.insert(name, AnyUniform::from_name(t, name, &self).unwrap());
        Ok(self.uniforms.get(name).unwrap().clone())
    }
    fn get_uniform(&self, name: &'static str) -> Result<AnyUniform, Error> { 
        match self.uniforms.get(name) {
            Some(u) => Ok(u.clone()),
            None => Err(Error::UniformError("cannot find uniform in hashmap"))
        } 
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
#[derive(Clone)]
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

#[derive(Clone)]
struct UniformI<const N: usize> { location: i32 }
impl<const N: usize> Uniform for UniformI<N> {
    fn new(location: i32) -> Self { Self{location} }
}
impl UniformI<1> {
    fn set(&self, x: i32) { unsafe { gl::Uniform1i(self.location, x); } }
}
impl UniformI<2> {
    fn set(&self, x: i32, y: i32) { unsafe { gl::Uniform2i(self.location, x, y); } }
}
impl UniformI<3> {
    fn set(&self, x: i32, y: i32, z: i32) { unsafe { gl::Uniform3i(self.location, x, y, z); } }
}
impl UniformI<4> {
    fn set(&self, x: i32, y: i32, z: i32, w: i32) { unsafe { gl::Uniform4i(self.location, x, y, z, w); } }
}

#[derive(Clone)]
enum AnyUniform { // this is sadly the only way to do this?
    F1(UniformF<1>),
    F2(UniformF<2>),
    F3(UniformF<3>),
    F4(UniformF<4>),
    I1(UniformI<1>),
    I2(UniformI<2>),
    I3(UniformI<3>),
    I4(UniformI<4>),
}
enum UniformType {
    F1,
    F2,
    F3,
    F4,
    I1, //1i
    I2,
    I3,
    I4,
}
impl AnyUniform {
    fn from_name(n: UniformType, name: &'static str, shader_program:&ShaderProgram) -> Result<Self, Error> {
        match n {
            UniformType::F1 => Ok(Self::F1(UniformF::<1>::from_name(name, shader_program).unwrap())),
            UniformType::F2 => Ok(Self::F2(UniformF::<2>::from_name(name, shader_program).unwrap())),
            UniformType::F3 => Ok(Self::F3(UniformF::<3>::from_name(name, shader_program).unwrap())),
            UniformType::F4 => Ok(Self::F4(UniformF::<4>::from_name(name, shader_program).unwrap())),
            UniformType::I1 => Ok(Self::I1(UniformI::<1>::from_name(name, shader_program).unwrap())),
            UniformType::I2 => Ok(Self::I2(UniformI::<2>::from_name(name, shader_program).unwrap())),
            UniformType::I3 => Ok(Self::I3(UniformI::<3>::from_name(name, shader_program).unwrap())),
            UniformType::I4 => Ok(Self::I4(UniformI::<4>::from_name(name, shader_program).unwrap())),

            _ => Err(Error::UniformError("no such uniform type"))
        }
    }
}

struct Texture {
    texture: u32,
    width:  u32,
    height: u32,
}
impl Texture {
    fn from_file(path: &'static str) -> Result<Self, Error> {
        let source = match std::fs::read(path) {
            Ok(d) => d,
            Err(_) => return Err(Error::TextureError(format!("couldn't find: {}", path)))
        };
        let img = match ImageReader::new(
            Cursor::new(source)
        ).with_guessed_format().unwrap().decode() {
            Ok(a) => a.to_rgba8(),
            Err(e) => return Err(Error::TextureError(format!("couldn't decode? image: {}", e)))
        };

        let (width, height) = img.dimensions();
        let mut texture: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr().cast()
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return Ok(Self { texture, width, height })
    }
    fn bind(&self, texture_unit: u32) { unsafe { 
        gl::ActiveTexture(gl::TEXTURE0+texture_unit);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
    } }
}

#[repr(C)]
struct Vertex {
    position:  [f32; 3],
    color:     [f32; 4],
    tex_coord: [f32; 2],
}
impl Vertex {
    fn new(position: [f32; 3], color: [f32; 4], tex_coord: [f32; 2]) -> Self {
        Self{position, color, tex_coord}
    }
}

pub struct Renderer {
    vbo: [VertexBufferObject; 2],
    vao: [VertexArrayObject; 2],
    ebo: [ElementBufferObject; 2],
    shader_program: Vec<ShaderProgram>,
    textures: HashMap<&'static str, Texture>,
    wireframe: bool
}
impl Renderer {
    pub fn init(window: &mut glfw::Window) -> Self {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        let vbo = [VertexBufferObject::new().unwrap(), VertexBufferObject::new().unwrap()];
        let vao = [VertexArrayObject::new().unwrap(), VertexArrayObject::new().unwrap()];
        let ebo = [ElementBufferObject::new().unwrap(), ElementBufferObject::new().unwrap()];
        
        Self::set_viewport(WINDOW_SIZE_X.try_into().unwrap(), WINDOW_SIZE_Y.try_into().unwrap());
        
        let vert_shader1 = Shader::from_file("src/bin/client/shaders/shader.vert", gl::VERTEX_SHADER).unwrap();
        let frag_shader1 = Shader::from_file("src/bin/client/shaders/shader.frag", gl::FRAGMENT_SHADER).unwrap();
        let mut shader_program= vec![
            ShaderProgram::create(vert_shader1, frag_shader1).unwrap()
        ];
        let u1 = shader_program[0].add_uniform(UniformType::I1, "texture1\0").unwrap();
        let u2 = shader_program[0].add_uniform(UniformType::I1, "texture2\0").unwrap();
        match (u1, u2) {
            (AnyUniform::I1(x), AnyUniform::I1(y)) => {x.set(0); y.set(0)},
            _ => panic!("how?")
        };
        let textures = HashMap::from([
            ("container", Texture::from_file("src/bin/client/textures/container.jpg").unwrap()),
            ("awesomeface", Texture::from_file("src/bin/client/textures/awesomeface.png").unwrap())
        ]);
        
        Self::set_texture_params();

        return Self{
            vbo,
            vao,
            ebo,
            shader_program,
            textures,
            wireframe: true,
        };
    }
    
    pub fn render(&mut self, triangles: Vec<Triangle>, text_coords: [[f32; 2]; 6]) -> Result<(), Error> {
        let mut t = 0;
        for (i, triangle) in triangles.iter().enumerate() {
            //let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
            let mut verts: Vec<Vertex> = vec![];
            for v in triangle.as_array() {
                //verts.push((v, [1.0, 0.0, 0.0, 1.0], text_coords[t]));
                verts.push(Vertex::new(v, [1.0, 0.0, 0.0, 1.0], text_coords[t]));
                t+=1;
            }
            //dbg!(&verts);
            
            self.vao[i].bind();
            self.vbo[i].bind();
            self.vbo[i].buffer(&verts[..], gl::STATIC_DRAW);
    
            //explain to gl how to read supplied vertices and save it to vao
            unsafe {
                let stride = size_of::<Vertex>() as i32;
                //vertices
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, position) as *const _);
                gl::EnableVertexAttribArray(0);
                //color
                gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, color) as *const _);
                gl::EnableVertexAttribArray(1);
                //texture
                gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, tex_coord) as *const _);
                gl::EnableVertexAttribArray(2);
            }
            self.ebo[i].bind();
            self.ebo[i].buffer_elements(vec![[0, 1, 2]], gl::STATIC_DRAW);
        }

        self.clear_color(crate::BACKGROUND_COLOR.as_tuple());
        self.clear();

        self.shader_program[0].use_program();

        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.shader_program[0].program, "texture1".as_ptr().cast()), 0); }
        self.textures.get("container").unwrap().bind(0);
        self.textures.get("awesomeface").unwrap().bind(1);
        self.vao[0].bind();
        self.ebo[0].bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());}

        self.shader_program[0].use_program();
        self.vao[1].bind();
        self.ebo[1].bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());}        
        return Ok(()) //TODO: return deltatime
    }

    fn clear_color(&self, bg_color: (f32, f32, f32, f32)) {
        unsafe { gl::ClearColor(bg_color.0, bg_color.1, bg_color.2, bg_color.3); }
    }
    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    }

    fn set_viewport(x: i32, y: i32) { unsafe { gl::Viewport(0, 0, x, y); } }
    fn set_texture_params() { unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());

    } }
    pub fn resize(&self, x: i32, y: i32) { 
        println!("resize: {:?}", (x, y));
        Self::set_viewport(x, y)
    }
    pub fn switch_wireframe(&mut self) {
        if self.wireframe { unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }; self.wireframe=false }
        else { unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }; self.wireframe=true }
    }
}