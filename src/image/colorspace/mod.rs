use self::{hsi::HSI, rgb::RGB, lab::Lab};

pub mod hsi;
pub mod lab;
pub mod rgb;

pub trait ColorSpace {
    fn to_rgb(&self) -> RGB;
    fn to_hsi(&self) -> HSI;
    fn to_lab(&self) -> Lab;
}
