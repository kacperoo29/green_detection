use super::{lab::Lab, hsi::HSI, ColorSpace};

#[derive(Clone)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl ColorSpace for RGB {
    fn to_rgb(&self) -> RGB {
        self.clone()
    }

    fn to_hsi(&self) -> HSI {
        let sum = self.red as f32 + self.green as f32 + self.blue as f32;
        let r = self.red as f32 / sum;
        let g = self.green as f32 / sum;
        let b = self.blue as f32 / sum;

        let inner = 0.5 * ((r - g) + (r - b)) / ((r - g).powf(2.0) + (r - b) * (g - b)).sqrt();
        let h = if b <= g {
            inner.acos()
        } else {
            2.0 * std::f32::consts::PI - inner.acos()
        } * 180.0 / std::f32::consts::PI;

        let s = 1.0 - 3.0 * (r.min(g).min(b)) / (r + g + b);
        let i = (self.red as f32 + self.green as f32 + self.blue as f32) / (3.0 * 255.0);

        HSI {
            hue: h,
            saturation: s,
            intensity: i,
            alpha: self.alpha as f32 / 255.0,
        }
    }

    fn to_lab(&self) -> Lab {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;

        let x = 0.412453 * r + 0.357580 * g + 0.180423 * b;
        let y = 0.212671 * r + 0.715160 * g + 0.072169 * b;
        let z = 0.019334 * r + 0.119193 * g + 0.950227 * b;

        let x = x / 0.950456;
        let y = y / 1.0;
        let z = z / 1.088754;

        let fx = match x {
            x if x > 0.008856 => x.powf(1.0 / 3.0),
            _ => 7.787 * x + 16.0 / 116.0,
        };

        let fy = match y {
            y if y > 0.008856 => y.powf(1.0 / 3.0),
            _ => 7.787 * y + 16.0 / 116.0,
        };

        let fz = match z {
            z if z > 0.008856 => z.powf(1.0 / 3.0),
            _ => 7.787 * z + 16.0 / 116.0,
        };

        Lab {
            l: 116.0 * fy - 16.0,
            a: 500.0 * (fx - fy),
            b: 200.0 * (fy - fz),
            alpha: self.alpha as f32 / 255.0,
        }
    }
}
