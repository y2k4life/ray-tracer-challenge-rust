use crate::float_eq;
use crate::Vector;
use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// An element with three floating point numbers ([`f64`]) which measure the
/// distance in space the point is from the origin.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    /// The distance the point is from the origin measured along the X axis.
    pub x: f64,
    /// The distance the point is from the origin measured along the Y axis.
    pub y: f64,
    /// The distance the point is from the origin measured along the Z axis.
    pub z: f64,
}

impl Point {
    /// Creates a `Point` in space measured from the origin using three [`f64`]
    /// numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustic_ray::Point;
    ///
    /// let p = Point::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(p.x, 1.0);
    /// assert_eq!(p.y, 2.0);
    /// assert_eq!(p.z, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x: {0:>10} y: {1:>10} z: {2:>10}",
            format!("{0:.5}", self.x),
            format!("{0:.5}", self.y),
            format!("{0:.5}", self.z)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector;

    // Chapter 1 Tuples, Points, and Vectors
    // page 4
    #[test]
    fn create_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn adding_vector_to_point_returns_a_vector() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn subtracting_a_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn negating_point() {
        let p = Point::new(1.0, -2.0, 3.0);

        assert_eq!(-p, Point::new(-1.0, 2.0, -3.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_point_by_scalar() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_eq!(a * 3.5, Point::new(3.5, -7.0, 10.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_a_point_by_a_fraction() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_eq!(a * 0.5, Point::new(0.5, -1.0, 1.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn dividing_a_point_by_a_scalar() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_eq!(a / 2.0, Point::new(0.5, -1.0, 1.5));
    }
}
