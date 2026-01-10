use std::io::Cursor;
use image::ImageReader;
use crate::Error;

pub struct Texture {
    texture: u32,
    width:  u32,
    height: u32,
}
impl Texture {
    pub fn from_file(path: &'static str) -> Result<Self, Error> {
        let source = match std::fs::read(path) {
            Ok(d) => d,
            Err(_) => return Err(Error::TextureError(format!("couldn't find: {}", path)))
        };
        let img = match ImageReader::new(
            Cursor::new(source)
        ).with_guessed_format().unwrap().decode() {
            Ok(a) => a.flipv().to_rgba8(),
            Err(e) => return Err(Error::TextureError(format!("couldn't decode? image: {}", e)))
        };

        let (width, height) = img.dimensions();
        let mut texture: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr().cast()
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return Ok(Self { texture, width, height })
    }
    pub fn bind(&self, texture_unit: u32) { unsafe { 
        gl::ActiveTexture(gl::TEXTURE0+texture_unit);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
    } }
}
