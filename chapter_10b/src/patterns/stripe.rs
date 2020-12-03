use super::Pattern;
use crate::{Color, Matrix, Point, IDENTITY};
use uuid::Uuid;

/// As the `x` coordinate changes, the pattern alternates between the colors.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stripe {
    id: Uuid,
    a: Color,
    b: Color,
    /// The transformation of the pattern.
    pub transform: Matrix,
}

impl Stripe {
    /// Create a new stripe pattern alternating between the two colors `a` and
    /// `b`.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Colors, patterns::Stripe};
    ///
    /// let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);
    ///
    /// assert_eq!(pattern.a, Colors::WHITE);
    /// assert_eq!(pattern.b, Colors::BLACK);
    /// ```
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe {
            id: Uuid::new_v4(),
            a,
            b,
            transform: IDENTITY,
        }
    }
}

impl Pattern for Stripe {
    fn id(&self) -> Uuid {
        self.id
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
    /// Chooses the color `a` or `b` for the given [`Point`].
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Colors, Point, patterns::Stripe};
    ///
    /// let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);
    ///
    /// assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
    /// assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), Colors::WHITE);
    /// assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), Colors::BLACK);
    /// assert_eq!(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), Colors::BLACK);
    /// assert_eq!(pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)), Colors::BLACK);
    /// assert_eq!(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), Colors::WHITE);
    ///```
    fn pattern_at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
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
    // Page 128
    #[test]
    fn creating_stripe_patter() {
        let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.a, Colors::WHITE);
        assert_eq!(pattern.b, Colors::BLACK);
    }

    // Chapter 10 Patterns
    // Page 129
    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 1.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 2.0, 0.0)), Colors::WHITE);
    }

    // Chapter 10 Patterns
    // Page 129
    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.0)), Colors::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 2.0)), Colors::WHITE);
    }

    // Chapter 10 Patterns
    // Page 129
    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.9, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.0, 0.0, 0.0)), Colors::BLACK);
        assert_eq!(
            pattern.pattern_at(Point::new(-0.1, 0.0, 0.0)),
            Colors::BLACK
        );
        assert_eq!(
            pattern.pattern_at(Point::new(-1.0, 0.0, 0.0)),
            Colors::BLACK
        );
        assert_eq!(
            pattern.pattern_at(Point::new(-1.1, 0.0, 0.0)),
            Colors::WHITE
        );
    }

    // Chapter 10 Patterns
    // Page 131
    // Removed
    // stripes_with_an_object_transformation() {

    // Chapter 10 Patterns
    // Page 131
    // removed
    // stripes_with_a_pattern_transformation() {

    // Chapter 10 Patterns
    // Page 131
    // stripes_with_both_an_object_and_a_pattern_transformation() {
}
