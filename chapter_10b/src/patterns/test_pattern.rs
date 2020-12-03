use super::Pattern;
use crate::{Color, Matrix, Point, IDENTITY};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TestPattern {
    id: Uuid,
    pub transform: Matrix,
}

impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern {
            id: Uuid::new_v4(),
            transform: IDENTITY,
        }
    }
}

impl Pattern for TestPattern {
    fn id(&self) -> Uuid {
        self.id
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }

    fn transform(&self) -> Matrix {
        self.transform
    }
}

impl Default for TestPattern {
    fn default() -> Self {
        Self::new()
    }
}
