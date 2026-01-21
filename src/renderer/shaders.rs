use crate::Error;

pub struct Shader(u32);
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
pub struct ShaderProgram {
    pub program: u32,
    _vertex_shader: Shader,
    _frag_shader:   Shader,
}
impl ShaderProgram {
    pub fn create(vertex_shader: Shader, frag_shader: Shader) -> Result<Self, Error> {
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
            return Ok(ShaderProgram { program, _vertex_shader: vertex_shader, _frag_shader: frag_shader })
        }
    }
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.program); }
    }
}