#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    const fn const_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    const fn const_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

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

    // built-in colors
    pub const BLACK: Color = Color::const_rgb(0, 0, 0);
    pub const WHITE: Color = Color::const_rgb(255, 255, 255);
    pub const RED: Color = Color::const_rgb(255, 0, 0);
    pub const GREEN: Color = Color::const_rgb(0, 255, 0);
    pub const BLUE: Color = Color::const_rgb(0, 0, 255);
    pub const YELLOW: Color = Color::const_rgb(255, 255, 0);
    pub const CYAN: Color = Color::const_rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::const_rgb(255, 0, 255);
    pub const GRAY: Color = Color::const_rgb(128, 128, 128);
    pub const DARK_GRAY: Color = Color::const_rgb(169, 169, 169);
    pub const LIGHT_GRAY: Color = Color::const_rgb(211, 211, 211);
    pub const TRANSPARENT: Color = Color::const_rgba(0, 0, 0, 0);
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

// built-in colors
