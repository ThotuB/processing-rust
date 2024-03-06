use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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

    pub fn hex_code(hex: &str) -> Result<Color, ColorParseError> {
        hex.parse()
    }

    pub fn hex(hex: u32) -> Color {
        Color {
            alpha: 255,
            red: ((hex >> 16) & 0xff) as u8,
            green: ((hex >> 8) & 0xff) as u8,
            blue: (hex & 0xff) as u8,
        }
    }

    pub fn with_alpha(self, alpha: u8) -> Color {
        Color { alpha, ..self }
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

impl From<Color> for [f32; 4] {
    fn from(val: Color) -> Self {
        [
            val.red as f32 / 255.0,
            val.green as f32 / 255.0,
            val.blue as f32 / 255.0,
            val.alpha as f32 / 255.0,
        ]
    }
}

impl From<Color> for u32 {
    fn from(val: Color) -> Self {
        (val.red as u32) << 24
            | (val.green as u32) << 16
            | (val.blue as u32) << 8
            | val.alpha as u32
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ColorParseError {
    #[error("invalid hex digit `{0}`")]
    InvalidHexDigit(char),
    #[error("invalid length")]
    InvalidLength,
}

macro_rules! match_hex {
    ($c:expr) => {
        match $c {
            b'0'..=b'9' => $c - b'0',
            b'a'..=b'f' => $c - b'a' + 10,
            b'A'..=b'F' => $c - b'A' + 10,
            _ => return Err(ColorParseError::InvalidHexDigit($c as char)),
        }
    };
    ($c1:expr, $c2:expr) => {
        match_hex!($c1) * 16 + match_hex!($c2)
    };
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = s.strip_prefix('#').unwrap_or(s);
        let s = s.as_bytes();
        if s.len() == 3 {
            let r = match_hex!(s[0]);
            let g = match_hex!(s[1]);
            let b = match_hex!(s[2]);

            Ok(Color::rgb(r * 17, g * 17, b * 17))
        } else if s.len() == 6 {
            let r = match_hex!(s[0], s[1]);
            let g = match_hex!(s[2], s[3]);
            let b = match_hex!(s[4], s[5]);

            Ok(Color::rgb(r, g, b))
        } else {
            Err(ColorParseError::InvalidLength)
        }
    }
}
