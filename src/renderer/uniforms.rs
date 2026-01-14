use crate::Error;
use crate::math::Matrix;
use crate::renderer::ShaderProgram;

pub struct Uniform(i32);
impl Uniform {
    pub fn from_name(name: &'static str, program: &ShaderProgram) -> Result<Self, Error> {
        let uni = unsafe{ gl::GetUniformLocation(program.program, name.as_ptr().cast())};
        if uni==-1 {return Err(Error::UniformError(name));}
        return Ok(Self(uni));
    }
    pub fn setf1(&self, x: f32) { unsafe { gl::Uniform1f(self.0, x); } }
    pub fn setf2(&self, x: f32, y: f32) { unsafe { gl::Uniform2f(self.0, x, y); } }
    pub fn setf3(&self, x: f32, y: f32, z: f32) { unsafe { gl::Uniform3f(self.0, x, y, z); } }
    pub fn setf4(&self, x: f32, y: f32, z: f32, w: f32) { unsafe { gl::Uniform4f(self.0, x, y, z, w); } }

    pub fn seti1(&self, x: i32) { unsafe { gl::Uniform1i(self.0, x); } }
    pub fn seti2(&self, x: i32, y: i32) { unsafe { gl::Uniform2i(self.0, x, y); } }
    pub fn seti3(&self, x: i32, y: i32, z: i32) { unsafe { gl::Uniform3i(self.0, x, y, z); } }
    pub fn seti4(&self, x: i32, y: i32, z: i32, w: i32) { unsafe { gl::Uniform4i(self.0, x, y, z, w); } }

    pub fn setmat3(&self, mat: Matrix<3, 3>) { unsafe { gl::UniformMatrix3fv(self.0, 1, gl::FALSE, mat.column_major().as_ptr().cast()); } }
    pub fn setmat4(&self, mat: Matrix<4, 4>) {
        unsafe { 
            gl::UniformMatrix4fv(
                self.0,
                1,
                gl::FALSE,
                mat.column_major().as_ptr().cast());
        } 
    }
}