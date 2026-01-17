use std::mem::offset_of;


use crate::{
    math::{Color, Vector, Vector3, vectors::Quaternion},
    renderer::{Vertex, buffers::{ElementBufferObject, VertexArrayObject, VertexBufferObject}, textures::{self, Texture}, uniforms::Uniform},
    vector
};

pub trait Object3D {
    fn draw(&self);
}

pub struct Triangle{
    world_pos: Vector3,

    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,
}
impl Triangle {
    pub fn basic(position: Vector3, color: Vector<4>, usage: gl::types::GLuint, ebo: Option<ElementBufferObject>) -> Self {
        let verts = [
            Vertex::from_vectors(vector!(0.5, 0.5, 0.0), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(0.5, -0.5, 0.0), color, vector!(1.0, -1.0)),
            Vertex::from_vectors(vector!(-0.5, 0.5, 0.0), color, vector!(-1.0, 1.0))
        ];
        Self::new(position, verts, usage, ebo)
    }
    pub fn new(position: Vector3, verts: [Vertex; 3], usage: u32, custom_ebo: Option<ElementBufferObject>) -> Self {
        let indices = [0, 1, 2];

        let vao = VertexArrayObject::new().unwrap();
        let vbo = VertexBufferObject::new().unwrap();
        
        vao.bind();
        vbo.bind();
        vbo.buffer(&verts, usage);
        
        //implement this as a function
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

        let ebo = match custom_ebo {
            Some(obj) => {obj},
            None => {
                let obj = ElementBufferObject::new().unwrap();
                obj.bind();
                obj.buffer_elements(vec![indices], usage);
                obj
            }
        };

        Self {
            world_pos: position,

            vao, vbo, ebo
        }
    }
}
impl Object3D for Triangle {
    fn draw(&self) {
        self.vao.bind();
        self.ebo.bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null()); }
    }
}

pub struct Rectangle {
    world_pos: Vector3,
    offset_uniform: Uniform,
    textures: Vec<Texture>,
    verts: [Vertex; 4],

    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,
}
impl Rectangle {
    pub fn new((x, y): (f32, f32), position: Vector3, color: Color, textures: Vec<Texture>, usage: gl::types::GLuint, offset_uniform: Uniform) -> Self {
        let verts = [
            Vertex::from_vectors(vector!(x, y, 0.0), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(-x, y, 0.0), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(x, -y, 0.0), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!(-x, -y, 0.0), color, vector!(0.0, 0.0)),
        ];
        let indices = vec![[1, 0, 2], [1, 2, 3]];

        let vao = VertexArrayObject::new().unwrap();
        let vbo = VertexBufferObject::new().unwrap();
        
        vao.bind();
        vbo.bind();
        vbo.buffer(&verts, usage);

        //implement this as a function
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

        let ebo = ElementBufferObject::new().unwrap();
        ebo.bind();
        ebo.buffer_elements(indices, usage);

        Self { 
            world_pos: position,
            verts,
            textures,
            offset_uniform,
            vao,vbo,ebo
        }
    }
    pub fn move_pos(&mut self, offset: Vector3) {
        self.world_pos = self.world_pos+offset;
    }
}
impl Object3D for Rectangle {
    fn draw(&self) {
        for (i, text) in self.textures.iter().enumerate() { text.bind(i as u32); }
        self.offset_uniform.setf3(self.world_pos.0[0], self.world_pos.0[1], self.world_pos.0[2]);
        self.vao.bind();
        self.ebo.bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()); }
    }
}