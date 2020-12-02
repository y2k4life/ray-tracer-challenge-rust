#[allow(unused_imports)]
use crate::Color;
use crate::{Canvas, Matrix, Point, Ray, World, IDENTITY};

/// Encapsulates the view and provides an interface for rendering the world
/// onto a [`Canvas`]. The [`Canvas`] is exactly one unit in front of the
/// `Camera`.
pub struct Camera {
    /// Horizontal size of the canvas.
    pub hsize: usize,
    /// Vertical size of the canvas.
    pub vsize: usize,
    /// Camera transformation matrix.
    pub transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    /// Construct a `Camera` with the give horizontal size (`hsize`), the given
    /// vertical size (`vsize`), the give field of view (`field_of_view`). The
    /// field of view is an angle that describes how much the camera can see.
    /// When the field of view is small, the view will be "zoomed in". Magnifying
    /// a smaller area of the scene.
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let mut half_width = half_view * aspect;
        let mut half_height = half_view;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            transform: IDENTITY,
            half_width,
            half_height,
            pixel_size,
        }
    }

    /// Returns a ray that starts at the camera and passes through the given
    /// `x` and `y` pixel on the canvas.
    pub fn ray_for_pixel(&mut self, px: f64, py: f64) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // the camera looks toward -z, so +x is to the *left*.
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // using the camera matrix, transform teh canvas point and the origin,
        // and then compute the ray's direction vector.
        // the canvas is at z: -1.
        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    /// Uses the camera to render an image of the given world. The `render`
    /// function creates a ray for each pixel of the canvas using the
    /// `ray_for_pixel` function. The computed [`Ray`] is then projected
    /// into the [`World`] using the `color_at` function of the [`World`] to get
    /// a [`Color`] for an object intersected by the [`Ray`] if there is one.
    pub fn render(&mut self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x as f64, y as f64);
                let color = world.color_at(ray);

                canvas.pixels[x][y] = color;
            }
        }

        canvas
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{float_eq, Color, Point, Transformation, Vector, World};

    use super::*;

    // Chapter 7 Making a Scene
    // Page 101
    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.transform, IDENTITY);
    }

    // Chapter 7 Making a Scene
    // Page 101
    #[test]
    fn the_pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!(float_eq(c.pixel_size, 0.01));
    }

    // Chapter 7 Making a Scene
    // Page 101
    #[test]
    fn the_pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(float_eq(c.pixel_size, 0.01));
    }

    // Chapter 7 Making a Scene
    // Page 103
    #[test]
    fn constructing_a_ray_through_the_center_of_canvas() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100.0, 50.0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    // Chapter 7 Making a Scene
    // Page 103
    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0.0, 0.0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    // Chapter 7 Making a Scene
    // Page 103
    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Transformation::new()
            .translate(0.0, -2.0, 5.0)
            .rotate_y(PI / 4.0)
            .build();
        let r = c.ray_for_pixel(100., 50.0);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    // Chapter 7 Making a Scene
    // Page 104
    #[test]
    pub fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        c.transform = Transformation::view_transform(from, to, up);
        let image = c.render(&w);

        assert_eq!(image.pixels[5][5], Color::new(0.38066, 0.47583, 0.2855));
    }
}
