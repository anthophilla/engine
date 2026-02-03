use crate::renderer::{Uniform, RenderError};

#[derive(Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}
impl Into<gl::types::GLuint> for ShaderType {
    fn into(self) -> gl::types::GLuint {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER
        }
    }
}
impl ToString for ShaderType {
    fn to_string(&self) -> String {
        match self {
            Self::Vertex => "VERTEX".to_string(),
            Self::Fragment => "FRAGMENT".to_string()
        }
    }
}

pub struct Shader(u32);
impl Shader {
    fn create(source: &[u8], shader_type: ShaderType) -> Result<Self, RenderError> {
        let shader = unsafe{ gl::CreateShader(shader_type.into()) };
        if shader == 0 {
            return Err(RenderError::ShaderError(format!("Couldnt create {} shader", &shader_type.to_string()))) }
        unsafe { 
            gl::ShaderSource(
                shader,
                1,
                &(source.as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
        
            gl::CompileShader(shader);
        }

        let mut success = 0;
        unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success); }
        if success == 0 {
            let mut info_buffer: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;

            unsafe {
                gl::GetShaderInfoLog(shader, 1024, &mut log_len, info_buffer.as_mut_ptr().cast());
                info_buffer.set_len(
                        log_len
                        .try_into()
                        .map_err(|_| RenderError::ShaderError(format!("try_into() on shader compile failed (wtf how?)")))?
                );
                return Err(RenderError::ShaderError(format!("Shader compile error: {}", String::from_utf8_lossy(&info_buffer))))
            };
        }

        return Ok(Self(shader))
    }

    pub fn from_file(path: &'static str, shader_type: ShaderType) -> Result<Self, RenderError> {
        let source = match std::fs::read(path) {
            Ok(d) => d,
            Err(_) => return Err(RenderError::ShaderError(format!("couldn't read: {path}")))
        };
        
        Self::create(&source, shader_type)
    }
}

//sturct Uniforms  TODO

pub struct ShaderProgram {
    pub program: u32,
    _vertex_shader: Shader,
    _frag_shader: Shader,

    pub perspective: Uniform,
    pub view:        Uniform,
    
    pub model_transform: Uniform,
    pub model_rotation:  Uniform,

    pub texture0: Uniform,
    pub texture1: Uniform,
}
impl ShaderProgram {
    pub fn create(vertex_shader: Shader, frag_shader: Shader) -> Result<Self, RenderError> {unsafe {
        let program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader.0.clone());
        gl::AttachShader(program, frag_shader.0.clone());

        gl::LinkProgram(program);

        let mut success= 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut info_buffer: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;

            gl::GetShaderInfoLog(program, 1024, &mut log_len, info_buffer.as_mut_ptr().cast());
            info_buffer.set_len(log_len
                .try_into()
                .map_err(|_| RenderError::ShaderError(format!("try_into() on shader program link failed (wtf how?)")))?
            );

            return Err(
                RenderError::ShaderError(format!("Shader program link error: {}", String::from_utf8_lossy(&info_buffer)))
            )
        }

        gl::DeleteShader(vertex_shader.0); // !! can lead to bugs
        gl::DeleteShader(frag_shader.0);

        return Ok(Self{
            program,
            _vertex_shader: vertex_shader,
            _frag_shader: frag_shader,

            perspective: Uniform::from_name("perspective\0", program)?,
            view: Uniform::from_name("view\0", program)?,

            model_transform: Uniform::from_name("model_trans\0", program)?,
            model_rotation: Uniform::from_name("model_rot\0", program)?,

            texture0: Uniform::from_name("texture1\0", program)?,
            texture1: Uniform::from_name("texture2\0", program)?,

        })
    }}
    pub fn use_program(&self) {unsafe { gl::UseProgram(self.program); }}
}