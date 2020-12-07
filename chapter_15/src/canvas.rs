use crate::Color;

/// A rectangular grid of pixels. The size of the canvas is determined by
/// its width and height
pub struct Canvas {
    /// Number of pixels wide the canvas is
    pub width: usize,
    /// Number of pixels height the canvas is
    pub height: usize,
    /// The pixels in the canvas
    // pub pixels: Vec<Vec<Color>>,
    pub pixels: Vec<Color>,
}

impl Canvas {
    /// Creates a new canvas with the height and width from the numbers provided.
    /// Each [`Color`] for the pixels in the canvas are black.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Canvas, Color};
    ///
    /// let c = Canvas::new(10, 20);
    ///
    /// assert_eq!(10, c.width);
    /// assert_eq!(20, c.height);
    ///
    /// for x in 0..10 {
    ///     for y in 0..20 {
    ///         assert_eq!(c.color(x, y), Color::new(0.0, 0.0, 0.0));
    ///     }
    /// }
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            //pixels: vec![vec![Color::new(0.0, 0.0, 0.0); height]; width],
            pixels: vec![Color::new(0.0, 0.0, 0.0); height * width],
        }
    }

    //    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
    //        self.pixels[x][y]
    //    }

    /// Output a canvas array for `self` to a string buffer in the PPM file
    /// format.
    pub fn canvas_to_ppm(&self) -> String {
        let mut buffer = ["P3", &format!("{} {}", self.width, self.height), "255"].join("\n");
        buffer.push('\n');

        let mut col_counter = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let p = &self.color(x, y);
                let red = Color::rgb_string(p.red);
                let green = Color::rgb_string(p.green);
                let blue = Color::rgb_string(p.blue);

                buffer.push_str(&Canvas::write_color(red, &mut col_counter));
                buffer.push_str(&Canvas::write_color(green, &mut col_counter));
                if x == self.width - 1 {
                    buffer.push_str(&Canvas::write_color(blue, &mut col_counter).trim());
                } else {
                    buffer.push_str(&Canvas::write_color(blue, &mut col_counter));
                }
            }
            buffer.push('\n');
            col_counter = 0;
        }
        buffer.push('\n');
        buffer
    }

    fn write_color(color: String, col_count: &mut usize) -> String {
        let mut color_buffer = String::new();
        if *col_count + color.len() > 70 {
            color_buffer.push('\n');
            *col_count = 0;
        }
        color_buffer.push_str(&color);
        *col_count += color.len();

        if *col_count + 4 > 70 {
            color_buffer.push('\n');
            *col_count = 0;
        } else {
            color_buffer.push(' ');
            *col_count += 1;
        }
        color_buffer
    }

    pub fn color(&self, x: usize, y: usize) -> Color {
        let i = x + y * self.width;
        self.pixels[i]
    }

    pub fn set_color(&mut self, x: usize, y: usize, c: Color) {
        let i = x + y * self.width;
        self.pixels[i] = c;
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
                assert_eq!(c.color(x, y), Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    // Chapter 2 Drawing on a Canvas
    // Page 19
    #[test]
    fn write_canvas() {
        let mut c = Canvas::new(10, 20);
        c.set_color(2, 3, Color::new(1.0, 0.0, 0.0));

        assert_eq!(c.color(2, 3), Color::new(1.0, 0.0, 0.0));
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
        c.set_color(0, 0, c1);
        c.set_color(2, 1, c2);
        c.set_color(4, 2, c3);
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
                c.set_color(x, y, Color::new(1.0, 0.8, 0.6));
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
