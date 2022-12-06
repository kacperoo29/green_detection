pub mod colorspace;
pub mod hsi;
pub mod lab;
pub mod rgb;

use self::colorspace::ColorSpace;

#[derive(Clone)]
pub struct Image<T>
where
    T: ColorSpace,
{
    data: Vec<T>,
    width: u32,
    height: u32,
}

impl<T> Image<T>
where
    T: ColorSpace,
{
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}
