use crate::core::errors::ColorError;

/// Struct that stores color in RGBA format (Red, Green, Blue, Alpha)
///
/// Each channel takes 1 byte ([`u8`]) and values from 0 to 255
/// Used to render gui elements like frames, labels, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8, pub g: u8, pub b: u8, pub a: u8,
}

impl Color {
    //Basic palette
    pub const WHITE:   Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK:   Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const RED:     Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN:   Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE:    Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const YELLOW:  Color = Color { r: 255, g: 255, b: 0, a: 255 };
    pub const CYAN:    Color = Color { r: 0, g: 255, b: 255, a: 255 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };

    //Special
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    //Grays
    pub const GRAY:       Color = Color { r: 128, g: 128, b: 128, a: 255 };
    pub const LIGHT_GRAY: Color = Color { r: 211, g: 211, b: 211, a: 255 };
    pub const DARK_GRAY:  Color = Color { r: 64, g: 64, b: 64, a: 255 };

    //Extended palette
    pub const ORANGE: Color = Color { r: 255, g: 165, b: 0, a: 255 };
    pub const PURPLE: Color = Color { r: 128, g: 0, b: 128, a: 255 };
    pub const PINK:   Color = Color { r: 255, g: 192, b: 203, a: 255 };
    pub const LIME:   Color = Color { r: 50, g: 205, b: 50, a: 255 };
    pub const TEAL:   Color = Color { r: 0, g: 128, b: 128, a: 255 };
    pub const SILVER: Color = Color { r: 192, g: 192, b: 192, a: 255 };
    pub const GOLD:   Color = Color { r: 255, g: 215, b: 0, a: 255 };

    ///Function to create color with alpha
    pub fn rgba(r:u8, g:u8, b:u8, a:u8) -> Self {
        Self{r,g,b,a}
    }

    ///Function to create color without alpha
    pub fn rgb(r:u8, g:u8, b:u8) -> Self {
        let a: u8 = 255; //alpha
        Self { r, g, b, a }
    }

    ///Function to make color lighter by given amount
    pub fn lighter(mut self, amount: u8) -> Self {
        self.r = self.r.saturating_add(amount);
        self.g = self.g.saturating_add(amount);
        self.b = self.b.saturating_add(amount);
        self
    }

    ///Function to make color darker by given amount
    pub fn darker(mut self, amount: u8) -> Self {
        self.r = self.r.saturating_sub(amount);
        self.g = self.g.saturating_sub(amount);
        self.b = self.b.saturating_sub(amount);
        self
    }

    ///Function to darken color by half
    pub fn darker_by_half(mut self) -> Self { 
        self.r = self.r.saturating_sub(self.r / 2);
        self.g = self.g.saturating_sub(self.g / 2);
        self.b = self.b.saturating_sub(self.b / 2);
        self
    }

    ///Function to lighten color by half
    pub fn lighter_by_half(mut self) -> Self {
        self.r = self.r.saturating_add(self.r / 2);
        self.g = self.g.saturating_add(self.g / 2);
        self.b = self.b.saturating_add(self.b / 2);
        self
    }

    ///Function to convert color to u32
    pub fn as_u32(&self) -> u32 {
        ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }

    ///Function to convert color to u32 with alpha
    pub fn as_u32_alpha(&self) -> u32 {
        ((self.r as u32) << 24) | 
        ((self.g as u32) << 16) | 
        ((self.b as u32) << 8)  | 
        (self.a as u32)
    }

    /// Function to convert from hex color to rgb/rgba
    /// Supports formats with and without "#" like: "#RRGGBBAA", "RRGGBBAA", "#RRGGBB", "RRGGBB"
    /// 
    /// Will return [`ColorError`] if it has wrong length or uses non-hex symbols (non 0-9 or A-F)
    /// 
    /// Example
    /// ```
    /// use haze_gui::core::color::Color;
    ///
    /// let rgb = Color::hex("#282828").unwrap(); //This will work here but its better to handle errors properly
    /// assert_eq!(rgb, Color::rgb(40, 40, 40));
    /// ```
    pub fn hex(hex_str: &str) -> Result<Self, ColorError> {
        let hex = hex_str.trim_start_matches('#');//Trimming # if there is

        if hex.len() != 6 && hex.len() != 8 { //Filtering by length
            return Err(ColorError::InvalidLength(hex.len()));
        }

        for c in hex.chars() { //Filtering if has nonhex chars
            if !c.is_ascii_hexdigit() {
                return Err(ColorError::InvalidHexCharacter(c));
            }
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().next().unwrap()))?; //Converting red channel
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().nth(2).unwrap()))?; //Converting green channel
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().nth(4).unwrap()))?; //Converting blue channel

        let a = if hex.len() == 8 {  //Converting alpha channel if there is
            u8::from_str_radix(&hex[6..8], 16).map_err(|_| ColorError::InvalidHexCharacter(hex.chars().nth(6).unwrap()))?
        } else {
            255 //If there are no alpha just give 255
        };

        Ok(Self::rgba(r, g, b, a))
    }
}

#[cfg(test)] //Simple tests
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
        let c = Color::rgba(255, 0, 0, 255);
        assert_eq!(c.as_u32_alpha(), 0xFF0000FF);
    }
}