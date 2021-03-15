//! Character and color management

use crossterm::style::Color;
use unicode_segmentation::UnicodeSegmentation;

/// # Pixel structure
/// contains color data and character data
#[derive(Clone, Eq, PartialEq)]
pub struct Pixel {
    /// Background color of the Pixel
    pub bg: Color,
    /// Foreground color of the Pixel
    pub fg: Color,
    /// Character of the Pixel
    pub chr: String,
}

impl Pixel {
    /// returns a tuple containing the background and foreground colors of a Pixel
    pub fn get_colors(&self) -> (Color, Color) {
        (self.fg, self.bg)
    }
}

/// Generate a pixel using a character, a foreground and background color
///
/// usage:
/// ```
/// use console_engine::pixel;
/// use console_engine::Color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_fbg("X", Color::Blue, Color::White));
/// ```
pub fn pxl_fbg(value: &str, fg: Color, bg: Color) -> Pixel {
    if value.graphemes(true).count() > 1 { panic!("Cannot set pixel to a string."); }

    Pixel { bg, fg, chr: value.to_string() }
}

/// Generate a pixel using a character and a foreground color.  
/// Background color is always black.
///
/// usage:
/// ```
/// use console_engine::pixel;
/// use console_engine::Color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_fg("X", Color::Cyan));
/// ```
pub fn pxl_fg(value: &str, fg: Color) -> Pixel {
    pxl_fbg(value, fg, Color::Reset)
}
/// Generate a pixel using a character and a background color.  
/// Foreground color is always White.
///
/// usage:
/// ```
/// use console_engine::pixel;
/// use console_engine::Color;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl_bg("X", Color::Magenta));
/// ```
pub fn pxl_bg(value: &str, bg: Color) -> Pixel {
    pxl_fbg(value, Color::Reset, bg)
}

/// Generate a pixel using a character  
/// Foreground color is always White.  
/// Background color is always black.
///
/// usage:
/// ```
/// use console_engine::pixel;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl("X"));
/// ```
pub fn pxl(value: &str) -> Pixel {
    pxl_fbg(value, Color::Reset, Color::Reset)
}
