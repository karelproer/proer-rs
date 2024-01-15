extern crate image;

pub enum SamplingMode {
    Nearset,
    Linear,
}

pub trait Texture {
    fn new<P: image::Pixel>(image: image::ImageBuffer<P, Vec<P::Subpixel>>, sampling_mode: SamplingMode) -> Self;
    fn empty<P: image::Pixel>(size: (u32, u32), sampling_mode: SamplingMode) -> Self;
}