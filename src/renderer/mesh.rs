use gl::types;

use crate::{
    math::{Color, Mat4, Vector3, Vector},
    renderer::{
        RenderError,
        Vertex,
        buffers::{ElementBufferObject, VertexArrayObject, VertexBufferObject},
        uniform::Uniform
    },
    vector
};

// pub enum MeshUsage{
//     Static
// }

pub trait Mesh {
    fn draw(&self, transform_uniform: &Uniform, rot_uniform: &Uniform);
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

    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,

    indices_count: i32,
}

impl StaticMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<i32>, world_position: Vector3) -> Result<Self, RenderError> {
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
            vao, vbo, ebo,
            indices_count
        })
    }

    /// creates a basic triangle
    pub fn triangle((x, y): (f32, f32), world_position: Vector3, color: Color) -> Result<Self, RenderError> {
        let vertices = vec![
            Vertex::from_vectors(vector!(-x/2.0, -y/2.0, 0.0), color, vector!(1.0, 1.0)),
            Vertex::from_vectors(vector!(0.0, y/2.0, 0.0), color, vector!(1.0, -1.0)),
            Vertex::from_vectors(vector!(x/2.0, -y/2.0, 0.0), color, vector!(-1.0, 1.0))
        ];

        let indices = vec![0, 1, 2];
        
        Ok(Self::new(vertices, indices, world_position)?)
    }
}
impl Mesh for StaticMesh {
    fn draw(&self, transform_uniform: &Uniform, rot_uniform: &Uniform) {
        // for (i, text) in self.textures.iter().enumerate() { text.bind(i as u32); }

        transform_uniform.setf3(&self.world_position);
        rot_uniform.setmat4(Mat4::IDENTITY);

        self.vao.bind();
        self.ebo.bind();

        unsafe { gl::DrawElements(gl::TRIANGLES, self.indices_count, gl::UNSIGNED_INT, std::ptr::null()); }
    }
}