use crate::float_eq;
use std::ops::{Add, Mul, Neg, Sub};

/// An element representing pixel on the drawing canvas.
///
/// A `Color` element is comprised of three floating point numbers ([`f64`])
/// ranging from 0.0 to 1.0. The three numbers represents the amount of red,
/// green, or blue the `Color` will have.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    /// Create a new [`Color'] using the `f64` values for red, green, and blue.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::Color;
    ///
    /// let c = Color::new(-0.5, 0.4, 1.7);
    ///
    /// assert_eq!(-0.5, c.red);
    /// assert_eq!(0.4, c.green);
    /// assert_eq!(1.7, c.blue);
    /// ```
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn from_tuple(color: (f64, f64, f64)) -> Self {
        Color {
            red: color.0,
            green: color.1,
            blue: color.2,
        }
    }

    pub fn rgb_string(color: f64) -> String {
        let mut rgb = color * 256.;
        if rgb < 0.0 {
            rgb = 0.0;
        }
        if rgb > 255.0 {
            rgb = 255.0;
        }
        let rgb = rgb as i64;
        format!("{}", rgb)
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        (
            Color::color_to_u8(self.red),
            Color::color_to_u8(self.green),
            Color::color_to_u8(self.blue),
        )
    }

    fn color_to_u8(c: f64) -> u8 {
        (255.0 * c) as u8
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self {
        Self {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: other.red * self,
            green: other.green * self,
            blue: other.blue * self,
        }
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Color {
            red: -self.red,
            green: -self.green,
            blue: -self.blue,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        float_eq(self.red, other.red)
            && float_eq(self.green, other.green)
            && float_eq(self.blue, other.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Chapter 2 Drawing on a Canvas
    // Page 16
    #[test]
    fn colors_are_a_red_green_blue_struct() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, c.red);
        assert_eq!(0.4, c.green);
        assert_eq!(1.7, c.blue);
    }

    // Chapter 2 Drawing on a Canvas
    // Page 17
    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    // Chapter 2 Drawing on a Canvas
    // Page 17
    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    // Chapter 2 Drawing on a Canvas
    // Page 17
    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    // Chapter 2 Drawing on a Canvas
    // Page 17
    #[test]
    fn multiplying_scalar_by_color() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(2.0 * c, Color::new(0.4, 0.6, 0.8));
    }

    // Chapter 2 Drawing on a Canvas
    // Page 17
    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
