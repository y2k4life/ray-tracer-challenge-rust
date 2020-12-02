use crate::{shapes::Shape, Color, Matrix, Point, IDENTITY};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stripe {
    id: Uuid,
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe {
            id: Uuid::new_v4(),
            a,
            b,
            transform: IDENTITY,
        }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: &dyn Shape, word_point: Point) -> Color {
        let object_point = object.transform().inverse() * word_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{shapes::Sphere, Colors, Point, Transformation};

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

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), Colors::WHITE);
    }

    // Chapter 10 Patterns
    // Page 129
    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), Colors::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), Colors::WHITE);
    }

    // Chapter 10 Patterns
    // Page 129
    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), Colors::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), Colors::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), Colors::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)), Colors::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), Colors::WHITE);
    }

    // Chapter 10 Patterns
    // Page 131
    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Sphere::new();
        object.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        let pattern = Stripe::new(Colors::WHITE, Colors::BLACK);

        let c = pattern.stripe_at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Colors::WHITE);
    }

    // Chapter 10 Patterns
    // Page 131
    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::new();
        let mut pattern = Stripe::new(Colors::WHITE, Colors::BLACK);
        pattern.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        let c = pattern.stripe_at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Colors::WHITE);
    }

    // Chapter 10 Patterns
    // Page 131
    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::new();
        object.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        let mut pattern = Stripe::new(Colors::WHITE, Colors::BLACK);
        pattern.transform = Transformation::new().translate(0.5, 0.0, 0.0).build();
        let c = pattern.stripe_at_object(&object, Point::new(2.5, 0.0, 0.0));

        assert_eq!(c, Colors::WHITE);
    }
}
