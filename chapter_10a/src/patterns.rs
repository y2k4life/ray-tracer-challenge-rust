//! Geometric rules that define how any given point in space out to be colored.
//! These patterns are stripes, gradients, rings, and checkers. Patterns are a
//! function that accepts a point in space and returns a color.
mod stripe;

pub use stripe::Stripe;
