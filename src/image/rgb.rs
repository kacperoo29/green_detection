use std::io::Cursor;

use image::io::Reader;

use super::{Image, colorspace::{rgb::RGB, ColorSpace, hsi::HSI, lab::Lab}};

impl Image<RGB> {
    pub fn new_with_data(data: Vec<u8>) -> Self {
        let reader = Reader::new(Cursor::new(&data[..]))
            .with_guessed_format()
            .expect("Couldn't guess file format.");

        let image = reader.decode().expect("Unable to decode image.");

        Self {
            data: image
                .to_rgba8()
                .to_vec()
                .chunks(4)
                .map(|pixel| RGB {
                    red: pixel[0],
                    green: pixel[1],
                    blue: pixel[2],
                    alpha: pixel[3],
                })
                .collect(),
            width: image.width(),
            height: image.height(),
        }
    }

    pub fn get_bitmap(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|pixel| vec![pixel.red, pixel.green, pixel.blue, pixel.alpha])
            .collect()
    }

    pub fn get_histogram(&self) -> [[u32; 3]; 256] {
        let mut histogram = [[0; 3]; 256];

        for pixel in self.data.iter() {
            histogram[pixel.red as usize][0] += 1;
            histogram[pixel.green as usize][1] += 1;
            histogram[pixel.blue as usize][2] += 1;
        }

        histogram
    }

    pub fn get_stretched_image(&self) -> Self {
        let histogram = self.get_histogram();
        let mut min = [0; 3];
        let mut max = [0; 3];

        for i in 0..256 {
            if histogram[i][0] > 0 && min[0] == 0 {
                min[0] = i as u8;
            }

            if histogram[i][1] > 0 && min[1] == 0 {
                min[1] = i as u8;
            }

            if histogram[i][2] > 0 && min[2] == 0 {
                min[2] = i as u8;
            }

            if histogram[i][0] > 0 {
                max[0] = i as u8;
            }

            if histogram[i][1] > 0 {
                max[1] = i as u8;
            }

            if histogram[i][2] > 0 {
                max[2] = i as u8;
            }
        }

        let mut data = Vec::new();
        for pixel in self.data.iter() {
            let red = (pixel.red - min[0]) as f32 / (max[0] - min[0]) as f32;
            let green = (pixel.green - min[1]) as f32 / (max[1] - min[1]) as f32;
            let blue = (pixel.blue - min[2]) as f32 / (max[2] - min[2]) as f32;

            data.push(RGB {
                red: (red * 255.0) as u8,
                green: (green * 255.0) as u8,
                blue: (blue * 255.0) as u8,
                alpha: pixel.alpha,
            });
        }

        Image {
            data,
            width: self.width,
            height: self.height,
        }
    }

    pub fn to_hsi(&self) -> Image<HSI> {
        let mut data = Vec::new();

        for pixel in self.data.iter() {
            data.push(pixel.to_hsi());
        }

        Image {
            data,
            width: self.width,
            height: self.height,
        }
    }

    pub fn to_lab(&self) -> Image<Lab> {
        let mut data = Vec::new();

        for pixel in self.data.iter() {
            data.push(pixel.to_lab());
        }

        Image {
            data,
            width: self.width,
            height: self.height,
        }
    }

    
    pub fn get_greenery_percentage(&self) -> f32 {
        let mut green = 0;
        
        for pixel in self.data.iter() {
            let rnorm = pixel.red as f32 / 255.0;
            let gnorm = pixel.green as f32 / 255.0;
            let bnorm = pixel.blue as f32 / 255.0;
            let rgb = rnorm + gnorm + bnorm;
            let r = rnorm / rgb;
            let g = gnorm / rgb;
            let b = bnorm / rgb;

            // https://iranarze.ir/wp-content/uploads/2018/03/E6087-IranArze.pdf
            let exg = 2.0 * g - r - b;
            let cive = 0.441 * r - 0.811 * g + 0.385 * b;
            let veg = g / (r.powf(0.667) * b.powf(0.333)) - 1.0;
            let idx = 0.36 * exg + 0.47 * cive + 0.17 * veg;
            if idx > 0.03 {
                green += 1;
            }

        }

        green as f32 / (self.width * self.height) as f32
    }

    pub fn test(&self) -> Self {
        let mut res = self.clone();
        for (i, pixel) in self.data.iter().enumerate() {
            let rnorm = pixel.red as f32 / 255.0;
            let gnorm = pixel.green as f32 / 255.0;
            let bnorm = pixel.blue as f32 / 255.0;
            let rgb = rnorm + gnorm + bnorm;
            let r = rnorm / rgb;
            let g = gnorm / rgb;
            let b = bnorm / rgb;

            let exg = 2.0 * g - r - b;
            let cive = 0.441 * r - 0.811 * g + 0.385 * b;
            let veg = g / (r.powf(0.667) * b.powf(0.333)) - 1.0;
            let idx = 0.36 * exg + 0.47 * cive + 0.17 * veg;

            if idx <= 0.03 {
                res.data[i] = RGB {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 255,
                }
            }

        }

        res
    }
}
