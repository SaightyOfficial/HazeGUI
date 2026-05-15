use crate::core::errors::ColorError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8, pub g: u8, pub b: u8, pub a: u8,
}

impl Color {
    //Basic
    pub const WHITE:   Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK:   Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const RED:     Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN:   Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE:    Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const YELLOW:  Color = Color { r: 255, g: 255, b: 0, a: 255 };
    pub const CYAN:    Color = Color { r: 0, g: 255, b: 255, a: 255 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };

    //Something
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    //Grays
    pub const GRAY:       Color = Color { r: 128, g: 128, b: 128, a: 255 };
    pub const LIGHT_GRAY: Color = Color { r: 211, g: 211, b: 211, a: 255 };
    pub const DARK_GRAY:  Color = Color { r: 64, g: 64, b: 64, a: 255 };

    //Extended
    pub const ORANGE: Color = Color { r: 255, g: 165, b: 0, a: 255 };
    pub const PURPLE: Color = Color { r: 128, g: 0, b: 128, a: 255 };
    pub const PINK:   Color = Color { r: 255, g: 192, b: 203, a: 255 };
    pub const LIME:   Color = Color { r: 50, g: 205, b: 50, a: 255 };
    pub const TEAL:   Color = Color { r: 0, g: 128, b: 128, a: 255 };
    pub const SILVER: Color = Color { r: 192, g: 192, b: 192, a: 255 };
    pub const GOLD:   Color = Color { r: 255, g: 215, b: 0, a: 255 };

    pub fn rgba(r:u8, g:u8, b:u8, a:u8) -> Self {
        Self{r,g,b,a}
    }
    pub fn rgb(r:u8, g:u8, b:u8) -> Self {
        let a:u8 = 255;
        Self { r, g, b, a }
    }

    pub fn lighter(mut self, amount: u8) -> Self {
        self.r = self.r.saturating_add(amount);
        self.g = self.g.saturating_add(amount);
        self.b = self.b.saturating_add(amount);
        self
    }

    pub fn darker(mut self, amount: u8) -> Self {
        self.r = self.r.saturating_sub(amount);
        self.g = self.g.saturating_sub(amount);
        self.b = self.b.saturating_sub(amount);
        self
    }

    pub fn as_u32(&self) -> u32 {
        ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }

    pub fn as_u32_bgra(&self) -> u32 {
        ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }

    pub fn as_u32_alpha(&self) -> u32 {
        ((self.r as u32) << 24) | 
        ((self.g as u32) << 16) | 
        ((self.b as u32) << 8)  | 
        (self.a as u32)
    }

    pub fn hex(hex_str: &str) -> Result<Self, ColorError> {
        let hex = hex_str.trim_start_matches('#');

        if hex.len() != 6 && hex.len() != 8 {
            return Err(ColorError::InvalidLength(hex.len()));
        }

        for c in hex.chars() {
            if !c.is_ascii_hexdigit() {
                return Err(ColorError::InvalidHexCharacter(c));
            }
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().next().unwrap()))?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().nth(2).unwrap()))?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().nth(4).unwrap()))?;

        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().nth(6).unwrap()))?
        } else {
            255
        };

        Ok(Self::rgba(r, g, b, a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_creation() {
        let c = Color::rgb(40, 40, 40);
        assert_eq!(c.r, 40);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_hex_parsing() {
        let c = Color::hex("#282828").unwrap();
        assert_eq!(c, Color::rgb(40, 40, 40));

        let err = Color::hex("#SMTHNH");
        assert!(err.is_err());
    }

    #[test]
    fn test_hex_alpha() {
        let c = Color::hex("#00000000").unwrap();
        assert_eq!(c.a, 0);
        
        let c2 = Color::hex("#FFFFFF80").unwrap();
        assert_eq!(c2.a, 128); 
    }

    #[test]
    fn test_as_u32_alpha() {
        let c = Color::rgba(255, 0, 0, 255); // Чистый красный
        // 255 << 24 | 0 << 16 | 0 << 8 | 255 = 0xFF0000FF
        assert_eq!(c.as_u32_alpha(), 0xFF0000FF);
    }
}