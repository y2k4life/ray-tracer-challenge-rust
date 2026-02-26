#[cfg(test)]
use super::Pattern;
#[cfg(test)]
use crate::{Color, Matrix, Point, IDENTITY};
#[cfg(test)]

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(test)]
pub struct TestPattern {
    id: u64,
    pub transform: Matrix,
}

#[cfg(test)]
impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern {
            id: crate::next_id(),
            transform: IDENTITY,
        }
    }
}

#[cfg(test)]
impl Pattern for TestPattern {
    fn id(&self) -> u64 {
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

#[cfg(test)]
impl Default for TestPattern {
    fn default() -> Self {
        Self::new()
    }
}
