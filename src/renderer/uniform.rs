use crate::{
    math::{Mat3, Mat4, Vector},
    renderer::{
        RenderError,
        shaders::ShaderProgram
    }
};

pub struct Uniform(i32);
impl Uniform {
    pub fn from_name(name: &'static str, program: &ShaderProgram) -> Result<Self, RenderError> {
        let uniform = unsafe { gl::GetUniformLocation(program.program, name.as_ptr().cast())};

        if uniform==-1 {return Err(RenderError::UniformError(format!("couldn't find uniform: '{name}' (maybe you forgot null byte)")))}
        return Ok(Self(uniform))
    }

    pub fn setf1(&self, x: f32) {unsafe { gl::Uniform1f(self.0, x); }}
    pub fn setf2(&self, v: &Vector<2>) {unsafe{ gl::Uniform2f(self.0, v[0], v[1]); }}
    pub fn setf3(&self, v: &Vector<3>) {unsafe{ gl::Uniform3f(self.0, v[0], v[1], v[2]); }}
    pub fn setf4(&self, v: &Vector<4>) {unsafe{ gl::Uniform4f(self.0, v[0], v[1], v[2], v[3]); }}

    pub fn seti1(&self, x: i32) {unsafe { gl::Uniform1i(self.0, x); }}
    pub fn seti2(&self, x: i32, y: i32) {unsafe { gl::Uniform2i(self.0, x, y); }}
    pub fn seti3(&self, x: i32, y: i32, z: i32) {unsafe { gl::Uniform3i(self.0, x, y, z); }}
    pub fn seti4(&self, x: i32, y: i32, z: i32, w: i32) {unsafe { gl::Uniform4i(self.0, x, y, z, w); }}

    pub fn setmat3(&self, mat: Mat3) {unsafe { gl::UniformMatrix3fv(self.0, 1, gl::FALSE, mat.as_column_major().as_ptr().cast()); }}
    pub fn setmat4(&self, mat: Mat4) {unsafe {
        gl::UniformMatrix4fv(
            self.0,
            1,
            gl::FALSE,
            mat.as_column_major().as_ptr().cast()
        );
    }}
}