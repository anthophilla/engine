use std::io::Cursor;

use image::{GenericImageView, ImageReader};

use crate::renderer::RenderError;

// #[derive(Clone, Copy)]
pub struct Texture {
    texture: u32,
    width: u32,
    height: u32,
}
impl Texture {
    pub fn from_file(path: &'static str) -> Result<Self, RenderError> {
        let source = std::fs::read(path).map_err(|_| RenderError::TextureError(format!("couldn't find: {path}")))?;
        
        let img = ImageReader::new(Cursor::new(source))
            .with_guessed_format()
            .map_err(|_| RenderError::TextureError(format!("couldn't guess format")))?
            .decode()
            .map_err(|e| RenderError::TextureError(format!("couldn't decode image{e}")))?
            .flipv()
            .to_rgba8();

        let (width, height) = img.dimensions();
        
        let mut texture: u32 = 0;
        unsafe{
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
                img.as_ptr().cast(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Self { texture, width, height })
    }
    pub fn bind(&self, texture_unit: u32) {unsafe {
        gl::ActiveTexture(gl::TEXTURE0+texture_unit);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
    }}
}