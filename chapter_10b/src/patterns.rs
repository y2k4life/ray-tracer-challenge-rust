//! Geometric rules that define how any given point in space out to be colored.
//! These patterns are stripes, gradients, rings, and checkers. Patterns are a
//! function that accepts a point in space and returns a color.
mod checkers;
mod gradient;
mod pattern;
mod ring;
mod stripe;
mod test_pattern;

pub use checkers::Checkers;
pub use gradient::Gradient;
pub use pattern::Pattern;
pub use ring::Ring;
pub use stripe::Stripe;
#[cfg(test)]
pub use test_pattern::TestPattern;
