use super::{lab::Lab, rgb::RGB, ColorSpace};

#[derive(Clone)]
pub struct HSI {
    pub hue: f32,
    pub saturation: f32,
    pub intensity: f32,
    pub alpha: f32,
}

impl ColorSpace for HSI {
    fn to_rgb(&self) -> RGB {
        let h = self.hue * std::f32::consts::PI / 180.0;
        let s = self.saturation;
        let i = self.intensity;

        let x = i * (1.0 - s);
        let y = i * (1.0 + s * h.cos() / f32::cos(std::f32::consts::PI / 3.0 - h));
        let z = 3.0 * i - (x + y);

        let r = match h {
            h if h < 2.0 * std::f32::consts::PI / 3.0 => y,
            h if h < 4.0 * std::f32::consts::PI / 3.0 => x,
            _ => z,
        };

        let g = match h {
            h if h < 2.0 * std::f32::consts::PI / 3.0 => z,
            h if h < 4.0 * std::f32::consts::PI / 3.0 => y,
            _ => x,
        };

        let b = match h {
            h if h < 2.0 * std::f32::consts::PI / 3.0 => x,
            h if h < 4.0 * std::f32::consts::PI / 3.0 => z,
            _ => y,
        };

        RGB {
            red: (r * 255.0) as u8,
            green: (g * 255.0) as u8,
            blue: (b * 255.0) as u8,
            alpha: (self.alpha * 255.0) as u8,
        }
    }

    fn to_hsi(&self) -> HSI {
        self.clone()
    }

    fn to_lab(&self) -> Lab {
        self.to_rgb().to_lab()
    }
}
