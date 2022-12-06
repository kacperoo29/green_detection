use super::{
    colorspace::{hsi::HSI, rgb::RGB, ColorSpace},
    Image,
};

impl Image<HSI> {
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

    pub fn get_intensity_histogram(&self) -> [u32; 256] {
        let mut histogram = [0; 256];

        for pixel in self.data.iter() {
            histogram[(pixel.intensity * 255.0).floor() as usize] += 1;
        }

        histogram
    }

    pub fn get_intensity_equalized_image(&self) -> Self {
        let histogram = self.get_intensity_histogram();
        let mut cdf = [0; 256];
        let mut min = 0;
        let mut sum = 0;

        for i in 0..256 {
            sum += histogram[i];
            cdf[i] = sum;

            if histogram[i] > 0 && min == 0 {
                min = i as u32;
            }
        }

        let mut data = Vec::new();
        for pixel in self.data.iter() {
            let intensity = (cdf[(pixel.intensity * 255.0).floor() as usize] - min) as f32
                / (self.width * self.height - min) as f32;

            data.push(HSI {
                hue: pixel.hue,
                saturation: pixel.saturation,
                intensity,
                alpha: pixel.alpha,
            });
        }

        Image {
            data,
            width: self.width,
            height: self.height,
        }
    }
}
