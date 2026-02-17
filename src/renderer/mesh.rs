// this code needs a good refactor

use crate::{
    math::{Color, Mat4, Quaternion, Vector, Vector3},
    renderer::{
        RenderError, Vertex, buffers::{ElementBufferObject, VertexArrayObject, VertexBufferObject}, texture::Texture, uniform::Uniform
    },
    vector
};

// pub enum MeshUsage{
//     Static
// }

pub trait Mesh {
    fn draw(&self, rot_uniform: &Uniform, trans_uniform: &Uniform);
}

// pub struct MeshPlaceHolder {
//     vertices: Vec<Vertex>,
//     indices: Vec<i32>,
//     world_position: Vector3,
//     usage: MeshUsage
// }
// impl MeshPlaceHolder {
//     pub fn new_static(vertices: Vec<Vertex>, indices: Vec<i32>, world_position: Vector3) -> Self {
//         Self { vertices, indices, world_position, usage: MeshUsage::Static }
//     }
// }

pub struct StaticMesh {
    world_position: Vector3,
    rotation: Quaternion,

    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,

    textures: Vec<Texture>,

    indices_count: i32,
}

impl StaticMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<i32>, world_position: Vector3, rotation: Quaternion, textures: Vec<Texture>) -> Result<Self, RenderError> {
        let vao = VertexArrayObject::new()?;
        let vbo = VertexBufferObject::new()?;
        vao.bind();
        vbo.bind();
        vbo.buffer(&vertices, gl::STATIC_DRAW);

        Vertex::set_attrib_pointers();

        let indices_count = indices.len().clone() as i32;

        let ebo = ElementBufferObject::new()?;
        ebo.bind();
        ebo.buffer_elements(indices, gl::STATIC_DRAW);

        Ok(Self{
            world_position,
            rotation,
            vao, vbo, ebo,
            textures,
            indices_count
        })
    }

    pub fn set_position(&mut self, pos: Vector3) {
        self.world_position = pos;
    }
    pub fn set_rotation(&mut self, rot: Quaternion) {
        self.rotation = rot;
    }

    /// creates a basic triangle with messed up uv's!!
    pub fn triangle((x, y): (f32, f32), world_position: Vector3, rotation: Quaternion, color: Color, textures: Vec<Texture>) -> Result<Self, RenderError> {
        let vertices = vec![
            Vertex::from_vectors(vector!(-x/2.0, -y/2.0, 0.0), color, vector!(-1.0, -1.0)),
            Vertex::from_vectors(vector!(0.0, y/2.0, 0.0), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(x/2.0, -y/2.0, 0.0), color, vector!(1.0, -1.0))
        ];

        let indices = vec![0, 1, 2];
        
        Ok(Self::new(vertices, indices, world_position, rotation, textures)?)
    }

    pub fn rectangle((x, y): (f32, f32), world_position: Vector3, rotation: Quaternion, color: Color, textures: Vec<Texture>) -> Result<Self, RenderError> {
        let vertices = vec![
            Vertex::from_vectors(vector!(x, y, 0.0), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(-x, y, 0.0), color, vector!(0.0, 1.0)),
            Vertex::from_vectors(vector!(x, -y, 0.0), color, vector!(1.0, 0.0)),
            Vertex::from_vectors(vector!(-x, -y, 0.0), color, vector!(0.0, 0.0)),
        ];
        let indices = vec![1, 0, 2, 1, 2, 3];

        Ok(Self::new(vertices, indices, world_position, rotation, textures)?)
    }

    //sorry in advance for the hardcode
    pub fn cube((x, y, z): (f32, f32, f32), world_position: Vector3, rotation: Quaternion, color: Color, textures: Vec<Texture>) -> Result<Self, RenderError> {
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
        let indices = vec![
             0,  1,  2,  3,  4,  5,
             6,  7,  8,  9, 10, 11,
            12, 13, 14, 15, 16, 17,
            18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29,
            30, 31, 32, 33, 34, 35,
        ];
        
        Ok(Self::new(vertices, indices, world_position, rotation, textures)?)
    }
}
impl Mesh for StaticMesh {
    fn draw(&self, rot_uniform: &Uniform, trans_uniform: &Uniform) {
        for (i, text) in self.textures.iter().enumerate() { text.bind(i as u32); }

        trans_uniform.setmat4(Mat4::translation_mat(self.world_position));
        rot_uniform.setmat4(Mat4::from(self.rotation));

        self.vao.bind();
        self.ebo.bind();

        unsafe { gl::DrawElements(gl::TRIANGLES, self.indices_count, gl::UNSIGNED_INT, std::ptr::null()); }
    }
}