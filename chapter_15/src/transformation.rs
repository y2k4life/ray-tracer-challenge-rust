use crate::{multiple_array, Matrix, Point, Vector};

/// Transformations are used to move and deform objects. The transformations
/// included are scale, translate, rotate, and shear.
///
/// Transformations work by creating a `Transformation` with `new` to start a
/// chain of transformations. After creating a `Transformation` call various
/// transformation functions (`scale`, `rotate_z`, etc.) on the returned
/// `Transformation`. To build a transformation [`Matrix`] from the chain of
/// transformations call the `build` function. Creating a [`Matrix`] calculates
/// the inverse of the [`Matrix`] which is expensive. Instead of creating a
/// matrix for each transformation the matrix is built for the complete transformation.
/// A `Transformation`s data is an array which starts as an identity array. Each
/// call to a transformation function updates the array of `self` by multiplying
/// the array with an array that performs the transformation. Each function builds on
/// pervious transformation functions.
///
/// For example, to build a transformation that `scales` and `rotates` along the
/// `y` axis build the transformation with these chain of commands
/// `Transformation::new().Scale(2.0, 2.0, 2.0).rotate_y(PI).build()`.
///
/// There is one `transformation` for the camera. To build this call the
/// 'view_transform' function.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transformation {
    data: [[f64; 4]; 4],
}

impl Transformation {
    /// Create a new `Transformation` as the start of a transformation chain
    /// to be performed on an object.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Transformation};
    /// use std::f64::consts::PI;
    ///
    /// // to rotate, scale, and translate an object
    /// // start by creating a new Transformation then
    /// // chian the rotate, scale, and translate
    /// let transform = Transformation::new()
    ///     .rotate_x(PI / 2.0)
    ///     .scale(5.0, 5.0, 5.0)
    ///     .translate(10.0, 5.0, 7.0)
    ///     .build();
    /// let p = Point::new(1.0, 0.0, 1.0);
    ///
    /// assert_eq!(transform * p, Point::new(15.0, 0.0, 7.0));
    /// ```
    pub fn new() -> Transformation {
        Transformation {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a matrix from the transformations data
    pub fn build(&self) -> Matrix {
        Matrix::new(self.data)
    }

    /// A transformation that moves a point. An inverse of a translation
    /// is a transformation that moves a point in reverse. Applying a
    /// translation to a vector will not change the vector. A vector is an
    /// arrow moving it around in space does not change the direction it
    /// points.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Transformation};
    ///
    /// let transform = Transformation::new()
    ///     .translate(5.0, -3.0, 2.0)
    ///     .build();
    /// let p = Point::new(-3.0, 4.0, 5.0);
    ///
    /// assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0));
    /// ```
    pub fn translate(self, x: f64, y: f64, z: f64) -> Transformation {
        let m = [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Transformation {
            data: multiple_array(m, self.data),
        }
    }

    /// A transformation that scales all points of an object for the give
    /// axes that don't have a `0` value. A positive number will move the points
    /// outward and negative number will move them inward. Scaling can be applied
    /// to vectors as well changing their length.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Transformation};
    ///
    /// let transform = Transformation::new()
    ///     .scale(2.0, 3.0, 4.0)
    ///     .build();
    /// let p = Point::new(-4.0, 6.0, 8.0);
    ///
    /// assert_eq!(transform * p, Point::new(-8.0, 18.0, 32.0));
    /// ```
    pub fn scale(self, x: f64, y: f64, z: f64) -> Transformation {
        let m = [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Transformation {
            data: multiple_array(m, self.data),
        }
    }

    /// Rotates an object around the `x` axis for the give number of radians
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Transformation};
    /// use std::f64::consts::PI;
    ///
    /// let p = Point::new(0.0, 1.0, 0.0);
    /// let half_quarter = Transformation::new()
    ///     .rotate_x(PI / 4.0)
    ///     .build();
    /// let full_quarter = Transformation::new()
    ///     .rotate_x(PI / 2.0)
    ///     .build();
    ///
    /// assert_eq!(
    ///     half_quarter * p,
    ///     Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
    /// );
    /// assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    /// ```
    pub fn rotate_x(self, r: f64) -> Transformation {
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Transformation {
            data: multiple_array(m, self.data),
        }
    }

    /// Rotates an object around the `y` axis for the give number of radians
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Transformation};
    /// use std::f64::consts::PI;
    ///
    /// let p = Point::new(0.0, 0.0, 1.0);
    /// let half_quarter = Transformation::new()
    ///     .rotate_y(PI / 4.0)
    ///     .build();
    /// let full_quarter = Transformation::new()
    ///     .rotate_y(PI / 2.0)
    ///     .build();
    ///
    /// assert_eq!(
    ///     half_quarter * p,
    ///     Point::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
    /// );
    /// assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    /// ```
    pub fn rotate_y(self, r: f64) -> Transformation {
        let m = [
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-(r.sin()), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Transformation {
            data: multiple_array(m, self.data),
        }
    }

    /// Rotates an object around the `z` axis for the give number of radians.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Transformation};
    /// use std::f64::consts::PI;
    ///
    /// let p = Point::new(0.0, 1.0, 0.0);
    /// let half_quarter = Transformation::new()
    ///     .rotate_z(PI / 4.0)
    ///     .build();
    /// let full_quarter = Transformation::new()
    ///     .rotate_z(PI / 2.0)
    ///     .build();
    ///
    /// assert_eq!(
    ///     half_quarter * p,
    ///     Point::new(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
    /// );
    /// assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    pub fn rotate_z(&self, r: f64) -> Transformation {
        let m = [
            [r.cos(), -(r.sin()), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Transformation {
            data: multiple_array(m, self.data),
        }
    }

    /// A transformation that makes straight lines slanted. Changes each component
    /// of an object in proportion to the other two components. The x component
    /// changes in proportion to y and z. The y component changes in proportion
    /// to x and z. The z component changes in proportion to x and y. What this
    /// mean for example is that the farther the y coordinate is from zero,
    /// the more the x value changes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Transformation};
    ///
    /// let transform = Transformation::new()
    ///     .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    ///     .build();
    /// let p = Point::new(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(transform * p, Point::new(5.0, 3.0, 4.0));
    /// ```
    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Transformation {
        let m = [
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Transformation {
            data: multiple_array(m, self.data),
        }
    }

    /// Create a transformation matrix that orients the world relative to
    /// the camera. Specify where you want the camera to be in the scene with
    /// the `from` parameter. A point in the scene the camera is pointing
    /// at the `to` parameter. A vector indication which direction is `up`.
    pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);
        let orientation = [
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let translation = Transformation::new().translate(-from.x, -from.y, -from.z);
        Matrix::new(multiple_array(orientation, translation.data))
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Transformation;
    use crate::{Matrix, Point, Vector, IDENTITY};
    use std::f64::consts::PI;

    #[test]
    // Chapter 4 Transformations
    // Page 45
    fn multiplying_by_a_translation_matrix() {
        let transform = Transformation::new().translate(5.0, -3.0, 2.0).build();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 45
    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transformation::new().translate(5.0, -3.0, 2.0);
        let inv = transform.build().inverse();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 45
    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transformation::new().translate(5.0, -3.0, 2.0).build();
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    // Chapter 4 Matrix Transformations
    // Page 46
    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Transformation::new().scale(2.0, 3.0, 4.0).build();
        let p = Point::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Point::new(-8.0, 18.0, 32.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 46
    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Transformation::new().scale(2.0, 3.0, 4.0).build();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Vector::new(-8.0, 18.0, 32.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 46
    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transformation::new().scale(2.0, 3.0, 4.0).build();
        let inv = transform.inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 47
    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Transformation::new().scale(-1.0, 1.0, 1.0).build();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 48
    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new().rotate_x(PI / 4.0).build();
        let full_quarter = Transformation::new().rotate_x(PI / 2.0).build();

        assert_eq!(
            half_quarter * p,
            Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 49
    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new().rotate_x(PI / 4.0).build();
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            Point::new(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt() / 2.0))
        );
    }

    // Chapter 4 Matrix Transformations
    // Page 49
    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Transformation::new().rotate_y(PI / 4.0).build();
        let full_quarter = Transformation::new().rotate_y(PI / 2.0).build();

        assert_eq!(
            half_quarter * p,
            Point::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 50
    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new().rotate_z(PI / 4.0).build();
        let full_quarter = Transformation::new().rotate_z(PI / 2.0).build();

        assert_eq!(
            half_quarter * p,
            Point::new(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 52
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Transformation::new()
            .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(5.0, 3.0, 4.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 52
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Transformation::new()
            .shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(6.0, 3.0, 4.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 52
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Transformation::new()
            .shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 5.0, 4.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 52
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Transformation::new()
            .shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 7.0, 4.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 52
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Transformation::new()
            .shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 3.0, 6.0));
    }

    // Chapter 4 Matrix Transformations
    // Page 52
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Transformation::new()
            .shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 3.0, 7.0));
    }

    // Chapter 7 Making a Scene
    // Page 98
    #[test]
    pub fn the_transformation_matrix_for_the_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transformation::view_transform(from, to, up);

        assert_eq!(t, IDENTITY);
    }

    // Chapter 7 Making a Scene
    // Page 98
    #[test]
    fn view_transformation_matrix_looking_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transformation::view_transform(from, to, up);

        assert_eq!(t, Transformation::new().scale(-1.0, 1.0, -1.0).build());
    }

    // Chapter 7 Making a Scene
    // Page 99
    #[test]
    fn view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transformation::view_transform(from, to, up);

        assert_eq!(t, Transformation::new().translate(0.0, 0.0, -8.0).build());
    }

    // Chapter 7 Making a Scene
    // Page 98
    #[test]
    #[rustfmt::skip]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = Transformation::view_transform(from, to, up);

        let e = Matrix::new([
            [-0.50709, 0.50709,  0.67612, -2.36643],
            [ 0.76772, 0.60609,  0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714,  0.00000],
            [ 0.00000, 0.00000,  0.00000,  1.00000],
        ]);

        assert_eq!(t, e);
    }
}
