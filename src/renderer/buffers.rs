use crate::renderer::{
    RenderError, Vertex
};

pub struct VertexArrayObject(gl::types::GLuint);
impl VertexArrayObject {
    pub fn new() -> Result<Self, RenderError> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        
        if vao!=0 { return Ok(Self(vao)) }
        else { Err(RenderError::VAOError) }
    }

    pub fn bind(&self) {unsafe { gl::BindVertexArray(self.0); }}
    pub fn _unbind() {unsafe { gl::BindVertexArray(0); }}
}

pub struct VertexBufferObject(gl::types::GLuint);
impl VertexBufferObject {
    pub fn new() -> Result<Self, RenderError> {
        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        
        if vbo!=0 { return Ok(Self(vbo)) }
        else { Err(RenderError::VBOError) }
    }

    pub fn bind(&self) {unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.0); }}
    pub fn _unbind() {unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }}
    
    pub fn buffer(&self, vertices: &[Vertex], usage: gl::types::GLenum) {unsafe {
        //dbg!(&vertices);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(vertices) as isize,
            vertices.as_ptr().cast(),
            usage,
        );
    }}
}

pub struct ElementBufferObject(gl::types::GLuint);
impl ElementBufferObject {
    pub fn new() -> Result<Self, RenderError> {
        let mut ebo = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }

        if ebo != 0 {return Ok(Self(ebo));}
        else {return Err(RenderError::EBOError);}
    }
    pub fn bind(&self) {unsafe {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.0);
    }}
    pub fn _unbind() {unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }}

    pub fn buffer_elements(&self, indices: Vec<i32>, usage: gl::types::GLenum) {unsafe {
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr().cast(),
            usage,
        );
    }}
}