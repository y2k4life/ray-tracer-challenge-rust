use super::Pattern;
use crate::{Color, Matrix, Point, IDENTITY};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Checkers {
    id: Uuid,
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Checkers {
        Checkers {
            id: Uuid::new_v4(),
            a,
            b,
            transform: IDENTITY,
        }
    }
}

impl Pattern for Checkers {
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
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Colors, Point};

    // Chapter 10 Patterns
    // Page 137
    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Checkers::new(Colors::WHITE, Colors::BLACK);
        
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(
            pattern.pattern_at(Point::new(0.99, 0.0, 0.0)),
            Colors::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Point::new(1.01, 0.0, 0.0)),
            Colors::BLACK
        );
    }

    // Chapter 10 Patterns
    // Page 137
    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Checkers::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.99, 0.0)),
            Colors::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 1.01, 0.708)),
            Colors::BLACK
        );
    }

    // Chapter 10 Patterns
    // Page 137
    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Checkers::new(Colors::WHITE, Colors::BLACK);
        
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 0.99)),
            Colors::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 1.01)),
            Colors::BLACK
        );
    }
}
