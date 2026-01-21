use std::mem::offset_of;

use crate::{
    math::{Color, Matrix4x4, Vector, Vector3, vectors::Quaternion},
    renderer::{
        Vertex,
        buffers::{ElementBufferObject, VertexArrayObject, VertexBufferObject},
        textures::Texture, uniforms::Uniform
    },
    vector
};

pub struct StaticMesh {
    textures: Vec<Texture>,
    indices_count: i32,

    world_position: Vector3,
    orientation: Quaternion,

    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,
}
impl StaticMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<i32>, world_position: Vector3, orientation: Quaternion, textures: Vec<Texture>, usage: gl::types::GLuint) -> Self {
        let vao = VertexArrayObject::new().unwrap();
        let vbo = VertexBufferObject::new().unwrap();
        
        vao.bind();
        vbo.bind();
        vbo.buffer(&vertices, usage);

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

        let indices_count = indices.len().clone() as i32;

        let ebo = ElementBufferObject::new().unwrap();
        ebo.bind();
        ebo.buffer_elements(indices, usage);

        Self {
            textures,
            world_position,
            orientation,
            indices_count,
            vao, vbo, ebo
        }
    }

    pub fn draw(&self, transform_uniform: &Uniform, model_uniform: &Uniform) {
        for (i, text) in self.textures.iter().enumerate() { text.bind(i as u32); }
        transform_uniform.setf3(self.world_position.0[0], self.world_position.0[1], self.world_position.0[2]);
        model_uniform.setmat4(self.orientation.to_matrix4x4());
        self.vao.bind();
        self.ebo.bind();
        //dbg!(&self.indices_count);
        // each triangle*3 vertices
        unsafe { gl::DrawElements(gl::TRIANGLES, self.indices_count, gl::UNSIGNED_INT, std::ptr::null()); }
    }
    pub fn translate(&mut self, pos: Vector3) {
        self.world_position = pos;
    }
    pub fn set_rotation(&mut self, rot: Quaternion) {
        self.orientation = rot;
    }
}

pub struct Triangle{
    world_pos: Vector3,
    orientation: Quaternion,
    pub mesh: StaticMesh,
}
impl Triangle {
    pub fn new((x, y): (f32, f32), position: Vector3, orientation: Quaternion, color: Color, textures: Vec<Texture>, usage: gl::types::GLuint) -> Self {
        let vertices = vec![
            Vertex::from_vectors(vector!(-x/2.0, -y/2.0, 0.0), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(0.0, y/2.0, 0.0), color, vector!(1.0, -1.0)),
            Vertex::from_vectors(vector!(x/2.0, -y/2.0, 0.0), color, vector!(-1.0, 1.0))
        ];
        let indices = vec![0, 1, 2];

        let mesh = StaticMesh::new(vertices, indices, position, orientation, textures, usage);

        Self { 
            world_pos: position,
            orientation,
            mesh
        }
    }
    pub fn update_mesh(&mut self) {
        self.mesh.translate(self.world_pos);
        self.mesh.set_rotation(self.orientation);
    }
    pub fn translate(&mut self, offset: Vector3) {
        self.world_pos = self.world_pos+offset;
        self.update_mesh();
    }
}

pub struct Rectangle {
    world_pos: Vector3,
    orientation: Quaternion,
    pub mesh: StaticMesh,
}
impl Rectangle {
    pub fn new((x, y): (f32, f32), position: Vector3, orientation: Quaternion, color: Color, textures: Vec<Texture>, usage: gl::types::GLuint) -> Self {
        let vertices = vec![
            Vertex::from_vectors(vector!(x, y, 0.0), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(-x, y, 0.0), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(x, -y, 0.0), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!(-x, -y, 0.0), color, vector!(0.0, 0.0)),
        ];
        let indices = vec![1, 0, 2, 1, 2, 3];

        let mesh = StaticMesh::new(vertices, indices, position, orientation, textures, usage);

        Self { 
            world_pos: position,
            orientation,
            mesh
        }
    }
    pub fn update_mesh(&mut self) {
        self.mesh.translate(self.world_pos);
        self.mesh.set_rotation(self.orientation);
    }
    pub fn translate(&mut self, offset: Vector3) {
        self.world_pos = self.world_pos+offset;
        self.update_mesh();
    }
    pub fn rotate(&mut self, _rot: Quaternion) {
        todo!("broken!!!");
        // self.orientation = self.orientation*rot;
        // self.update_mesh();
    }
}

pub struct Cube {
    world_pos: Vector3,
    orientation: Quaternion,
    pub mesh: StaticMesh,
}
impl Cube {
    pub fn new((x, y, z): (f32, f32, f32), position: Vector3, orientation: Quaternion, color: Color, textures: Vec<Texture>, usage: gl::types::GLuint) -> Self {
        //copy fucking pasted from chatgpt
        let vertices = vec![
            // back face
            Vertex::from_vectors(vector!(-x, -y, -z), color, vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!( x, -y, -z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!( x,  y, -z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!( x,  y, -z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(-x,  y, -z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(-x, -y, -z), color, vector!(0.0, 0.0)),

            // front face
            Vertex::from_vectors(vector!(-x, -y,  z), color, vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!( x, -y,  z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!( x,  y,  z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!( x,  y,  z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(-x,  y,  z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(-x, -y,  z), color, vector!(0.0, 0.0)),

            // left face
            Vertex::from_vectors(vector!(-x,  y,  z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!(-x,  y, -z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(-x, -y, -z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(-x, -y, -z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(-x, -y,  z), color, vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!(-x,  y,  z), color, vector!(1.0, 0.0)),

            // right face
            Vertex::from_vectors(vector!( x,  y,  z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!( x,  y, -z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!( x, -y, -z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!( x, -y, -z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!( x, -y,  z), color, vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!( x,  y,  z), color, vector!(1.0, 0.0)),

            // bottom face
            Vertex::from_vectors(vector!(-x, -y, -z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!( x, -y, -z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!( x, -y,  z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!( x, -y,  z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!(-x, -y,  z), color, vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!(-x, -y, -z), color, vector!(0.0, 1.0)),

            // top face
            Vertex::from_vectors(vector!(-x,  y, -z), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!( x,  y, -z), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!( x,  y,  z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!( x,  y,  z), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!(-x,  y,  z), color, vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!(-x,  y, -z), color, vector!(0.0, 1.0)),
        ];
        //should be a better way to do this
        let indices = vec![
             0,  1,  2,  3,  4,  5,
             6,  7,  8,  9, 10, 11,
            12, 13, 14, 15, 16, 17,
            18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29,
            30, 31, 32, 33, 34, 35,
        ];

        let mesh = StaticMesh::new(vertices, indices, position, orientation, textures, usage);

        Self { 
            world_pos: position,
            orientation,
            mesh
        }
    }
    pub fn update_mesh(&mut self) {
        self.mesh.translate(self.world_pos);
        self.mesh.set_rotation(self.orientation);
    }
    pub fn translate(&mut self, offset: Vector3) {
        self.world_pos = self.world_pos+offset;
        self.update_mesh();
    }
    pub fn rotate(&mut self, _rot: Quaternion) {
        todo!("broken!!!");
        // self.orientation = self.orientation*rot;
        // self.update_mesh();
    }
}

pub struct AxesArrows {
    vao: VertexArrayObject,
}
impl AxesArrows {
    pub fn new() -> Self {
        let vertices = vec![
            //x
            Vertex::from_vectors(vector!(0.0, 0.0, 0.0), vector!(1.0, 0.0, 0.0, 1.0), vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!(1.0, 0.0, 0.0), vector!(1.0, 0.0, 0.0, 1.0), vector!(0.0, 0.0)),
            //y
            Vertex::from_vectors(vector!(0.0, 0.0, 0.0), vector!(0.0, 1.0, 0.0, 1.0), vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!(0.0, 1.0, 0.0), vector!(0.0, 1.0, 0.0, 1.0), vector!(0.0, 0.0)),
            //z
            Vertex::from_vectors(vector!(0.0, 0.0, 0.0), vector!(0.0, 0.0, 1.0, 1.0), vector!(0.0, 0.0)),
            Vertex::from_vectors(vector!(0.0, 0.0, 1.0), vector!(0.0, 0.0, 1.0, 1.0), vector!(0.0, 0.0)),
        ];

        let vao = VertexArrayObject::new().unwrap();
        let vbo = VertexBufferObject::new().unwrap();
        
        vao.bind();
        vbo.bind();
        vbo.buffer(&vertices, gl::STATIC_DRAW);

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
        Self{ vao }
    }
    pub fn draw(&self, model_uniform: &Uniform) {
        self.vao.bind();
        model_uniform.setmat4(Matrix4x4::IDENTITY);
        unsafe { gl::DrawArrays(gl::LINES, 0, 6);}
        self.vao._unbind();
    }
}