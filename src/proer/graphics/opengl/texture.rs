use super::super::texture;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl texture::Texture for Texture {
    fn new<P: image::Pixel>(image: image::ImageBuffer<P, Vec<P::Subpixel>>, sampling_mode: texture::SamplingMode) -> Self {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            let color_type= match P::COLOR_MODEL { "RGB" => gl::RGB, "RGBA" => gl::RGBA, _ => panic!("Unsupported image format!") };
            gl::TexImage2D(gl::TEXTURE_2D, 0, color_type.try_into().unwrap(), image.width().try_into().unwrap(), image.height().try_into().unwrap(), 0, color_type, gl::UNSIGNED_BYTE, image.into_raw().as_ptr() as *const std::ffi::c_void);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, match sampling_mode { texture::SamplingMode::Nearset => gl::NEAREST, texture::SamplingMode::Linear => gl::LINEAR }.try_into().unwrap() );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, match sampling_mode { texture::SamplingMode::Nearset => gl::LINEAR_MIPMAP_NEAREST, texture::SamplingMode::Linear => gl::LINEAR_MIPMAP_LINEAR }.try_into().unwrap() );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Self { id: texture }
        }
    }

}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}