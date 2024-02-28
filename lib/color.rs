#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn transparent() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }

    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.red as f32 / 255.0,
            self.green as f32 / 255.0,
            self.blue as f32 / 255.0,
            self.alpha as f32 / 255.0,
        )
    }

    pub fn as_u8(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }

    pub fn as_u32(&self) -> u32 {
        (self.red as u32) << 24
            | (self.green as u32) << 16
            | (self.blue as u32) << 8
            | self.alpha as u32
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    fn from(color: Color) -> (f32, f32, f32, f32) {
        color.as_f32()
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> [f32; 4] {
        let (r, g, b, a) = color.as_f32();
        [r, g, b, a]
    }
}
