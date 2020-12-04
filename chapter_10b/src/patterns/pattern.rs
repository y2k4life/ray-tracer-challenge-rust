#[allow(unused_imports)]
use crate::Transformation;
use crate::{shapes::Shape, Color, Matrix, Point};
use std::fmt;
use uuid::Uuid;

pub trait Pattern: Send + fmt::Debug {
    /// Get the unique identifier for a pattern.
    fn id(&self) -> Uuid;

    /// Test if `other` is equal to `self` by comparing their `id`'s.
    fn pattern_eq(&self, other: &dyn Pattern) -> bool {
        self.id() == other.id()
    }

    /// Returns a pattern's [`Transformation`] [`'Matrix`].
    fn transform(&self) -> Matrix;

    /// Sets a pattern's [`Transformation`] [`'Matrix`].
    fn set_transform(&mut self, transform: Matrix);

    /// Determine a color from a pattern a particular point on the pattern.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{
    ///     shapes::Sphere, patterns::Pattern, patterns::Checkers, Color, Colors,
    ///     Point, Transformation
    /// };
    ///
    /// let mut object = Sphere::new();
    /// object.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
    /// let pattern = Checkers::new(Colors::WHITE, Colors::BLACK);
    /// let c = pattern.pattern_at_shape(&object, Point::new(2.0, 3.0, 4.0));
    ///
    /// assert_eq!(c, Colors::WHITE);
    ///```
    fn pattern_at(&self, point: Point) -> Color;

    /// Determines color the point of the object using the following steps.
    ///
    /// 1. Convert the point from world space to object space
    /// 2. Convert the object space point to *pattern space*
    /// 3. Get the color of the pattern by calling `stripe_at` with the
    /// point on the pattern.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{
    ///     shapes::Sphere, patterns::Pattern, patterns::Checkers, Color, Colors,
    ///     Point, Transformation
    /// };
    ///
    /// let mut object = Sphere::new();
    /// object.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
    /// let pattern = Checkers::new(Colors::WHITE, Colors::BLACK);
    /// let c = pattern.pattern_at_shape(&object, Point::new(2.0, 3.0, 4.0));
    ///
    /// assert_eq!(c, Colors::WHITE);
    /// ```
    fn pattern_at_shape(&self, object: &dyn Shape, word_point: Point) -> Color {
        let object_point = object.transform().inverse() * word_point;
        let pattern_point = self.transform().inverse() * object_point;
        self.pattern_at(pattern_point)
    }
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Box<dyn Pattern>) -> bool {
        self.pattern_eq(other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::{patterns::TestPattern, shapes::Sphere, Transformation, IDENTITY};

    use super::*;

    // Chapter 10 Patterns
    // Page 133
    #[test]
    fn the_default_pattern_transformation() {
        let pattern = TestPattern::new();

        assert_eq!(pattern.transform(), IDENTITY);
    }

    // Chapter 10 Patterns
    // Page 133
    #[test]
    fn assign_a_transformation() {
        let mut pattern = TestPattern::new();
        pattern.set_transform(Transformation::new().translate(1.0, 2.0, 3.0).build());

        assert_eq!(
            pattern.transform(),
            Transformation::new().translate(1.0, 2.0, 3.0).build()
        );
    }

    // Chapter 10 Patterns
    // Page 131
    #[test]
    fn a_pattern_with_an_object_transformation() {
        let mut object = Sphere::new();
        object.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        let pattern = TestPattern::new();

        let c = pattern.pattern_at_shape(&object, Point::new(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    // Chapter 10 Patterns
    // Page 131
    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::new();
        let mut pattern = TestPattern::new();
        pattern.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        let c = pattern.pattern_at_shape(&object, Point::new(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    // Chapter 10 Patterns
    // Page 131
    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::new();
        object.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        let mut pattern = TestPattern::new();
        pattern.transform = Transformation::new().translate(0.5, 1.0, 1.5).build();
        let c = pattern.pattern_at_shape(&object, Point::new(2.5, 3.0, 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
