use super::{colorspace::{lab::Lab, ColorSpace, rgb::RGB}, Image};

impl Image<Lab> {
    pub fn a_histogram(&self) -> [u32; 256] {
        let mut histogram = [0; 256];

        for pixel in self.data.iter() {
            histogram[(pixel.a + 128.0) as usize] += 1;
        }

        histogram
    }

    pub fn to_rgb(&self) -> Image<RGB> {
        let mut data = Vec::new();

        for pixel in self.data.iter() {
            data.push(pixel.to_rgb());
        }

        Image {
            data,
            width: self.width,
            height: self.height,
        }
    }
}
