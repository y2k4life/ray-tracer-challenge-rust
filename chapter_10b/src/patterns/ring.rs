use super::Pattern;
use crate::{Color, Matrix, Point, IDENTITY};
use uuid::Uuid;

/// A ring pattern depending on the `x` and `z` dimensions to decide which 
/// [`Color`] to return.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ring {
    id: Uuid,
    a: Color,
    b: Color,
    /// The transformation of the pattern.
    pub transform: Matrix,
}

impl Ring {
    /// Create a new ring pattern using the [`Color`] `a` and `b`.
    pub fn new(a: Color, b: Color) -> Ring {
        Ring {
            id: Uuid::new_v4(),
            a,
            b,
            transform: IDENTITY,
        }
    }
}

impl Pattern for Ring {
    fn id(&self) -> Uuid {
        self.id
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        let x = (point.x * 100.0).round() / 100.0;
        let z = (point.z * 100.0).round() / 100.0;
        let t = (x.powi(2) + z.powi(2)).sqrt().floor();
        if t % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Colors;

    #[test]
    fn a_ring_should_extend_both_x_and_z() {
        let pattern = Ring::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.0, 0.0, 0.0)), Colors::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.0)), Colors::BLACK);
        assert_eq!(
            pattern.pattern_at(Point::new(0.708, 0.0, 0.708)),
            Colors::BLACK
        );
    }
}
