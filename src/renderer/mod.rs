mod buffers;
mod shaders;
pub mod textures;
mod uniforms;
pub mod objects;
pub mod camera;

//use std::collections::HashMap;

use shaders::{Shader, ShaderProgram};
use textures::Texture;
use uniforms::Uniform;

use crate::{
    Error, WINDOW_SIZE_X, WINDOW_SIZE_Y, game::Player, math::{
        Color, Vector, Vector3, Vector4, matrix::Matrix4x4, perspective, vectors::Quaternion
    }, renderer::{
        camera::Camera,
        objects::{Rectangle, StaticMesh}
    }, vector
};

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    position:  [f32; 3],
    color:     [f32; 4],
    tex_coord: [f32; 2],
}
impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 4], tex_coord: [f32; 2]) -> Self {
        Self{position, color, tex_coord}
    }
    pub fn from_vectors(position: Vector3, color: Color, tex_coord: Vector<2>) -> Self {
        Self::new(position.as_array(), color.as_array(), tex_coord.as_array())
    }
}

pub struct Renderer {
    shader_program: Vec<ShaderProgram>,
    uniforms: Vec<Uniform>,
    rectangles: Vec<Rectangle>,
    //textures: HashMap<&'static str, Texture>,
    wireframe: bool
}
impl Renderer {
    pub fn init(window: &mut glfw::Window) -> Self {
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        Self::set_viewport(WINDOW_SIZE_X.try_into().unwrap(), WINDOW_SIZE_Y.try_into().unwrap());
        
        let vert_shader1 = Shader::from_file("src/shaders/shader.vert", gl::VERTEX_SHADER).unwrap();
        let frag_shader1 = Shader::from_file("src/shaders/shader.frag", gl::FRAGMENT_SHADER).unwrap();
        let shader_program= vec![
            ShaderProgram::create(vert_shader1, frag_shader1).unwrap()
        ];

        let uniforms = vec![
            Uniform::from_name("texture1\0", &shader_program[0]).unwrap(),
            Uniform::from_name("texture2\0", &shader_program[0]).unwrap(),
            Uniform::from_name("model\0", &shader_program[0]).unwrap(),
            Uniform::from_name("view\0", &shader_program[0]).unwrap(),
            Uniform::from_name("perspective\0", &shader_program[0]).unwrap(),
            Uniform::from_name("transform\0", &shader_program[0]).unwrap(),
        ];

        // let textures = HashMap::from([
        //     ("container", Texture::from_file("src/textures/container.jpg").unwrap()),
        //     ("awesomeface", Texture::from_file("src/textures/awesomeface.png").unwrap())
        // ]);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
        Self::set_texture_params();

        let rectangles: Vec<Rectangle> = vec![
            Rectangle::new(
                (0.5, 0.5),
                vector!(0.0, 0.0, 1.0),
                Quaternion::from_angle_vect(0.0, vector!(0.0, 0.0, 0.0)),
                vector!(1.0, 0.0, 0.0, 1.0),
                vec![
                    Texture::from_file("src/textures/container.jpg").unwrap(),
                    Texture::from_file("src/textures/awesomeface.png").unwrap()
                ],
                gl::STATIC_DRAW
            )
        ];

        return Self{
            shader_program,
            uniforms,
            rectangles,
            wireframe: true,
        };
    }
    
    pub fn render(&mut self, meshes: &Vec<StaticMesh>, player: &mut Player) -> Result<(), Error> {
        
        self.clear_color(crate::BACKGROUND_COLOR.as_array());
        self.clear();
        
        self.shader_program[0].use_program();
        
        self.uniforms[0].seti1(0);
        self.uniforms[1].seti1(1);

        //dbg!(player.get_camera_world_position());
        let view = Matrix4x4::translation_mat(player.get_camera_world_position());
        
        self.uniforms[3].setmat4(view);
        self.uniforms[4].setmat4(player.camera.perspective);

        for mesh in meshes {
            //mesh.translate(vector!(0.0, 0.0, -0.01));
            //mesh.set_rotation(rot);
            //rect.rotate(rot); //broken
            //rect.mesh.draw(&self.uniforms[5], &self.uniforms[2]);
            mesh.draw(&self.uniforms[5], &self.uniforms[2]);
        }

        return Ok(())
    }

    fn clear_color(&self, bg_color: [f32; 4]) {
        unsafe { gl::ClearColor(bg_color[0], bg_color[1], bg_color[2], bg_color[3]); }
    }
    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
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