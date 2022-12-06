use super::{ColorSpace, rgb::RGB, hsi::HSI};

#[derive(Clone)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}

impl ColorSpace for Lab {
    fn to_rgb(&self) -> RGB {
        let l = self.l;
        let a = self.a;
        let b = self.b;

        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;

        let x = match fx {
            x if x > 0.206893 => 0.950456 * fx.powf(3.0),
            _ => 0.950456 * (fx - 16.0 / 116.0) / 7.787,
        };

        let y = match fy {
            y if y > 0.206893 => fy.powf(3.0),
            _ => (fy - 16.0 / 116.0) / 7.787,
        };

        let z = match fz {
            z if z > 0.206893 => 1.088754 * fz.powf(3.0),
            _ => 1.088754 * (fz - 16.0 / 116.0) / 7.787,
        };

        let r = 3.240479 * x - 1.537150 * y - 0.498535 * z;
        let g = -0.969256 * x + 1.875992 * y + 0.041556 * z;
        let b = 0.055648 * x - 0.204043 * y + 1.057311 * z;

        RGB {
            red: (r * 255.0) as u8,
            green: (g * 255.0) as u8,
            blue: (b * 255.0) as u8,
            alpha: (self.alpha * 255.0) as u8,
        }
    }

    fn to_hsi(&self) -> HSI {
        self.to_rgb().to_hsi()
    }

    fn to_lab(&self) -> Lab {
        self.clone()
    }
}
