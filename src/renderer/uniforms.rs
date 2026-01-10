use crate::Error;
use crate::renderer::ShaderProgram;

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
pub struct UniformF<const N: usize> { location: i32 }
impl<const N: usize> Uniform for UniformF<N> {
    fn new(location: i32) -> Self { Self{location} }
}

impl UniformF<1> {
    pub fn set(&self, x: f32) { unsafe { gl::Uniform1f(self.location, x); } }
}
impl UniformF<2> {
    pub fn set(&self, x: f32, y: f32) { unsafe { gl::Uniform2f(self.location, x, y); } }
}
impl UniformF<3> {
    pub fn set(&self, x: f32, y: f32, z: f32) { unsafe { gl::Uniform3f(self.location, x, y, z); } }
}
impl UniformF<4> {
    pub fn set(&self, x: f32, y: f32, z: f32, w: f32) { unsafe { gl::Uniform4f(self.location, x, y, z, w); } }
}

#[derive(Clone)]
pub struct UniformI<const N: usize> { location: i32 }
impl<const N: usize> Uniform for UniformI<N> {
    fn new(location: i32) -> Self { Self{location} }
}
impl UniformI<1> {
    pub fn set(&self, x: i32) { unsafe { gl::Uniform1i(self.location, x); } }
}
impl UniformI<2> {
    pub fn set(&self, x: i32, y: i32) { unsafe { gl::Uniform2i(self.location, x, y); } }
}
impl UniformI<3> {
    pub fn set(&self, x: i32, y: i32, z: i32) { unsafe { gl::Uniform3i(self.location, x, y, z); } }
}
impl UniformI<4> {
    pub fn set(&self, x: i32, y: i32, z: i32, w: i32) { unsafe { gl::Uniform4i(self.location, x, y, z, w); } }
}

#[derive(Clone)]
pub enum AnyUniform { // this is sadly the only way to do this?
    F1(UniformF<1>),
    F2(UniformF<2>),
    F3(UniformF<3>),
    F4(UniformF<4>),
    I1(UniformI<1>),
    I2(UniformI<2>),
    I3(UniformI<3>),
    I4(UniformI<4>),
}
pub enum UniformType {
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
    pub fn from_name(n: UniformType, name: &'static str, shader_program:&ShaderProgram) -> Result<Self, Error> {
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