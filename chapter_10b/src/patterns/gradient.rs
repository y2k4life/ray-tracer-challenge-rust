use super::Pattern;
use crate::{Color, Matrix, Point, IDENTITY};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gradient {
    id: Uuid,
    a: Color,
    b: Color,
    pub transform: Matrix,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        Gradient {
            id: Uuid::new_v4(),
            a,
            b,
            transform: IDENTITY,
        }
    }
}

impl Pattern for Gradient {
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
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }
}

#[cfg(test)]
mod tests {
    use crate::Colors;
    use super::*;

    // Chapter 10 Patterns
    // Page 135
    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient::new(Colors::WHITE, Colors::BLACK);
        
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(
            pattern.pattern_at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
