mod buffers;
mod shaders;
mod textures;
mod uniforms;

use std::collections::HashMap;
use std::f32::consts::PI;
use std::mem::offset_of;

use buffers::{VertexArrayObject, VertexBufferObject, ElementBufferObject};
use shaders::{Shader, ShaderProgram};
use textures::Texture;
use uniforms::Uniform;

use crate::math::matrix::Matrix4x4;
use crate::{Error, vector};
use crate::math::{Triangle, Vector, Vector4};
use crate::math::vectors::Quaternion;
use crate::{WINDOW_SIZE_X, WINDOW_SIZE_Y};

#[repr(C)]
struct Vertex {
    position:  [f32; 3],
    color:     [f32; 4],
    tex_coord: [f32; 2],
}
impl Vertex {
    fn new(position: [f32; 3], color: [f32; 4], tex_coord: [f32; 2]) -> Self {
        Self{position, color, tex_coord}
    }
}

pub struct Renderer {
    vbo: [VertexBufferObject; 2],
    vao: [VertexArrayObject; 2],
    ebo: [ElementBufferObject; 2],
    shader_program: Vec<ShaderProgram>,
    uniforms: Vec<Uniform>,
    textures: HashMap<&'static str, Texture>,
    wireframe: bool
}
impl Renderer {
    pub fn init(window: &mut glfw::Window) -> Self {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        let vbo = [VertexBufferObject::new().unwrap(), VertexBufferObject::new().unwrap()];
        let vao = [VertexArrayObject::new().unwrap(), VertexArrayObject::new().unwrap()];
        let ebo = [ElementBufferObject::new().unwrap(), ElementBufferObject::new().unwrap()];
        
        Self::set_viewport(WINDOW_SIZE_X.try_into().unwrap(), WINDOW_SIZE_Y.try_into().unwrap());
        
        let vert_shader1 = Shader::from_file("src/shaders/shader.vert", gl::VERTEX_SHADER).unwrap();
        let frag_shader1 = Shader::from_file("src/shaders/shader.frag", gl::FRAGMENT_SHADER).unwrap();
        let mut shader_program= vec![
            ShaderProgram::create(vert_shader1, frag_shader1).unwrap()
        ];

        let uniforms = vec![
            Uniform::from_name("texture1\0", &shader_program[0]).unwrap(),
            Uniform::from_name("texture2\0", &shader_program[0]).unwrap(),
            Uniform::from_name("transform\0", &shader_program[0]).unwrap(),
        ];

        let textures = HashMap::from([
            ("container", Texture::from_file("src/textures/container.jpg").unwrap()),
            ("awesomeface", Texture::from_file("src/textures/awesomeface.png").unwrap())
        ]);
        Self::set_texture_params();

        return Self{
            vbo,
            vao,
            ebo,
            shader_program,
            uniforms,
            textures,
            wireframe: true,
        };
    }
    
    pub fn render(&mut self, triangles: Vec<Triangle>, text_coords: [[f32; 2]; 6]) -> Result<(), Error> {
        let mut t = 0;
        for (i, triangle) in triangles.iter().enumerate() {
            //let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
            let mut verts: Vec<Vertex> = vec![];
            for v in triangle.as_array() {
                //verts.push((v, [1.0, 0.0, 0.0, 1.0], text_coords[t]));
                verts.push(Vertex::new(v, [1.0, 0.0, 0.0, 1.0], text_coords[t]));
                t+=1;
            }
            //dbg!(&verts);
            
            self.vao[i].bind();
            self.vbo[i].bind();
            self.vbo[i].buffer(&verts[..], gl::STATIC_DRAW);
    
            //explain to gl how to read supplied vertices and save it to vao
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
            self.ebo[i].bind();
            self.ebo[i].buffer_elements(vec![[0, 1, 2]], gl::STATIC_DRAW);
        }

        self.clear_color(crate::BACKGROUND_COLOR.as_array());
        self.clear();

        self.shader_program[0].use_program();

        self.textures.get("container").unwrap().bind(0);
        self.textures.get("awesomeface").unwrap().bind(1);

        self.uniforms[0].seti1(0);
        self.uniforms[1].seti1(1);

        let rot = Quaternion::from(vector!(PI/2.0, 0.0, 0.0, 1.0)).to_matrix4x4();
        self.uniforms[2].setmat4(rot);

        self.vao[0].bind();
        self.ebo[0].bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());}

        self.shader_program[0].use_program();
        self.vao[1].bind();
        self.ebo[1].bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());}        
        return Ok(()) //TODO: return deltatime
    }

    fn clear_color(&self, bg_color: [f32; 4]) {
        unsafe { gl::ClearColor(bg_color[0], bg_color[1], bg_color[2], bg_color[3]); }
    }
    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    }

    fn set_viewport(x: i32, y: i32) { unsafe { gl::Viewport(0, 0, x, y); } }
    fn set_texture_params() { unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());

    } }
    pub fn resize(&self, x: i32, y: i32) { 
        println!("resize: {:?}", (x, y));
        Self::set_viewport(x, y)
    }
    pub fn switch_wireframe(&mut self) {
        if self.wireframe { unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }; self.wireframe=false }
        else { unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }; self.wireframe=true }
    }
}