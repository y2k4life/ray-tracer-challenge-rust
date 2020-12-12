use crate::float_eq;
use std::ops::{Add, Mul, Neg, Sub};

/// An element representing pixel on the drawing canvas.
///
/// A `Color` element is comprised of three floating point numbers ([`f64`])
/// ranging from 0.0 to 1.0. The three numbers represents the factor of red,
/// green, or blue the `Color` will have.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    /// The amount of `red` is in the `color`.
    pub red: f64,
    /// The amount of `green` is in the `color`.
    pub green: f64,
    /// The amount of `blue` is in the `color`.
    pub blue: f64,
}

impl Color {
    /// Create a new `Color` using the `f64` values for red, green, and blue.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::Color;
    ///
    /// let c = Color::new(0.5, 0.4, 1.7);
    ///
    /// assert_eq!(0.5, c.red);
    /// assert_eq!(0.4, c.green);
    /// assert_eq!(1.7, c.blue);
    /// ```
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    /// Create a new color form [`u8`] values.
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{float_eq, Color};
    ///
    /// let c = Color::from_u8(128, 102, 179);
    ///
    /// assert!(float_eq(0.5019, c.red));
    /// assert!(float_eq(0.4, c.green));
    /// assert!(float_eq(0.70196, c.blue));
    /// ```
    pub fn from_u8(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: red as f64 / 255.0,
            green: green as f64 / 255.0,
            blue: blue as f64 / 255.0,
        }
    }

    /// Get an array of the  parts of a `Color` as [`u8`] in string format. The
    /// returned array is `["rrr", "ggg", "bbb"]
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::Color;
    ///
    /// let c = Color::new(0.5, 0.4, 0.7);
    /// let a = c.rgb_string_array();
    ///
    /// assert_eq!(a[0], "128");
    /// assert_eq!(a[1], "102");
    /// assert_eq!(a[2], "179");
    /// ```
    pub fn rgb_string_array(&self) -> [String; 3] {
        [
            format!("{}", (Self::clip_color(self.red) * 256.0) as u8),
            format!("{}", (Self::clip_color(self.green) * 256.0) as u8),
            format!("{}", (Self::clip_color(self.blue) * 256.0) as u8),
        ]
    }

    fn clip_color(color: f64) -> f64 {
        if color < 0.0 {
            0.0
        } else if color > 1.0 {
            1.0
        } else {
            color
        }
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

    fn mul(self, rhs: Self) -> Self {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self * rhs.red,
            green: self * rhs.green,
            blue: self * rhs.blue,
        }
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            red: -self.red,
            green: -self.green,
            blue: -self.blue,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
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
