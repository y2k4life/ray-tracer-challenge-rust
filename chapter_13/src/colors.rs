use crate::Color;

/// A pallette of primary colors
pub struct Colors;

impl Colors {
    /// The color black
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    /// The color white.
    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    /// The color red.
    pub const RED: Color = Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    };
}
