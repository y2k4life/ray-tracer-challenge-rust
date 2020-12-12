use std::ops::{Index, IndexMut};

use crate::Color;

const MAXIMUM_PPM_LINE_LENGTH: usize = 70;

/// A grid of pixels. The size of the canvas is determined by its width and height.
///
/// The pixels are store in a linear 1D array (`x + y * width`).
pub struct Canvas {
    /// Width of the canvas.
    width: usize,
    /// Height of the canvas.
    height: usize,
    
    pixels: Vec<Color>,
}

impl Canvas {
    /// Creates a new canvas with the given `height` and `width`. Each pixel will
    /// have a [`Color`] of black.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Canvas, Color};
    ///
    /// let c = Canvas::new(10, 20);
    ///
    /// for x in 0..10 {
    ///     for y in 0..20 {
    ///         assert_eq!(c.pixel_at(x, y), Color::new(0.0, 0.0, 0.0));
    ///     }
    /// }
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); height * width],
        }
    }

    /// Output the canvas buffer to a string buffer in the PPM file format.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Canvas, Color};
    /// 
    /// let mut c = Canvas::new(5, 3);
    /// let c1 = Color::new(1.5, 0.0, 0.0);
    /// let c2 = Color::new(0.0, 0.5, 0.0);
    /// let c3 = Color::new(-0.5, 0.0, 1.0);
    /// c.write_pixel(0, 0, c1);
    /// c.write_pixel(2, 1, c2);
    /// c.write_pixel(4, 2, c3);
    /// let actual = c.canvas_to_ppm();
    /// let split = actual.split("\n").collect::<Vec<_>>();
    /// 
    /// assert_eq!("P3", split[0]);
    /// assert_eq!("5 3", split[1]);
    /// assert_eq!("255", split[2]);
    /// assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", split[3]);
    /// assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", split[4]);
    /// assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", split[5]);
    /// ```
    pub fn canvas_to_ppm(&self) -> String {
        let mut buffer = ["P3", &format!("{} {}", self.width, self.height), "255"].join("\n");
        buffer.push('\n');

        let mut col_counter = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);

                for c in pixel.rgb_string_array().iter() {
                    if col_counter + c.len() + 1 > MAXIMUM_PPM_LINE_LENGTH {
                        buffer += "\n";
                        col_counter = 0;
                    }
                    if col_counter > 0 {
                        buffer += " ";
                    }
                    buffer += c;
                    col_counter += c.len() + 1;
                }
            }
            buffer.push('\n');
            col_counter = 0;
        }
        buffer.push('\n');
        buffer
    }

    /// Returns the [`Color`] of a pixel on the canvas at the specified `x` and
    /// `y` coordinates.
    ///
    /// Example
    /// ```
    /// use rustic_ray::{Canvas, Color};
    ///
    /// let mut c = Canvas::new(10, 20);
    /// c.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
    /// let color = c.pixel_at(2,3);
    ///
    /// assert_eq!(color, Color::new(1.0, 0.0, 0.0));
    /// ```
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let i = x + y * self.width;
        self.pixels[i]
    }

    /// Write a pixel to the canvas at the specified `x` and `y` coordinates
    /// having the specified [`Color`].
        ///
    /// Example
    /// ```
    /// use rustic_ray::{Canvas, Color};
    /// 
    /// let mut c = Canvas::new(10, 20);
    /// c.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
    /// let color = c.pixel_at(2,3);
    ///
    /// assert_eq!(color, Color::new(1.0, 0.0, 0.0));
    /// ```
    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let i = x + y * self.width;
        self.pixels[i] = c;
    }
}

/// Returns the pixel [`Color`] at the `index` of the `Canvas`.
impl Index<usize> for Canvas {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

/// Returns a mutable pixel [`Color`] at the `index` of the `Canvas`.
impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    // Chapter 2 Drawing on a Canvas
    // Page 19
    #[test]
    fn create_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(10, c.width);
        assert_eq!(20, c.height);
        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c.pixel_at(x, y), Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    // Chapter 2 Drawing on a Canvas
    // Page 19
    #[test]
    fn write_canvas() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));

        assert_eq!(c.pixel_at(2, 3), Color::new(1.0, 0.0, 0.0));
    }

    // Chapter 2 Drawing on a Canvas
    // Page 21 to 22
    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let actual = c.canvas_to_ppm();
        let split = actual.split("\n").collect::<Vec<_>>();

        assert_eq!("P3", split[0]);
        assert_eq!("5 3", split[1]);
        assert_eq!("255", split[2]);
    }

    // Chapter 2 Drawing on a Canvas
    // Page 21
    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let actual = c.canvas_to_ppm();
        let split = actual.split("\n").collect::<Vec<_>>();

        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", split[3]);
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", split[4]);
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", split[5]);
    }

    // Chapter 2 Drawing on a Canvas
    // Page 22
    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }
        let actual = c.canvas_to_ppm();

        let split = actual.split("\n").collect::<Vec<_>>();

        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            split[3]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            split[4]
        );
        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            split[5]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            split[6]
        );
    }
}
