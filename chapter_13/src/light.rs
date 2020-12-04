use crate::{Color, Point};

/// A light source with no size, existing at a single point in space.
///
/// A `PointLight` is defined by its position in space and the intensity or how
/// bright the light it is. The intensity also describes the color of the
/// light source.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    /// Brightness and color of the light
    pub intensity: Color,
    /// Position in space
    pub position: Point,
}

impl PointLight {
    /// Creates a new `PointLight` at the give [`Point`] with the given
    /// intensity and color.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Color, PointLight, Point};
    ///
    /// let intensity = Color::new(1.0, 1.0, 1.0);
    /// let position = Point::new(0.0, 0.0, 0.0);
    /// let light = PointLight::new(position, intensity);
    ///
    /// assert_eq!(light.position, position);
    /// assert_eq!(light.intensity, intensity);
    /// ```
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Point};

    /// Chapter 6 Light and Shading
    /// Page 84
    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
