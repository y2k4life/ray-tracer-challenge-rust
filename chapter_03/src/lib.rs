//! Rustic Ray is as ray tracer library based on the book The Ray Tracer
//! Challenge by Jamis Buck
mod canvas;
mod color;
mod matrix;
mod point;
mod vector;

pub use crate::canvas::Canvas;
pub use crate::color::Color;
pub use crate::matrix::Matrix;
pub use crate::matrix::IDENTITY;
pub use crate::point::Point;
pub use crate::vector::Vector;

use std::cmp::Ordering;

/// Error value used for comparing floating point number
pub const EPSILON: f64 = 0.0001;

/// Compare two floating point numbers to determine if they are
/// approximately equal
pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

/// Compare two floating point numbers to determine if `a` is equal, less, or
/// greater than `b`.
pub fn float_cmp(a: f64, b: f64) -> Ordering {
    if float_eq(a, b) {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals() {
        assert!(float_eq(1.111113, 1.111115));
    }

    #[test]
    fn equals2() {
        assert!(float_eq(0.21804511278195488, 0.21804999999999999));
    }

    #[test]
    fn equals3() {
        assert!(float_eq(0.0, 0.00000000000000006123233995736766));
    }

    #[test]
    fn less_than() {
        assert_eq!(float_cmp(4.5, 6.0), Ordering::Less);
    }

    #[test]
    fn greater_than() {
        assert_eq!(float_cmp(6.0, 4.5), Ordering::Greater);
    }
}
