//! Contains various shapes used in a scene. The shapes are [`Sphere`] and
//! [`Plane`].
mod plane;
mod shape;
mod sphere;
mod test_shape;

pub use plane::Plane;
pub use shape::Shape;
pub use sphere::Sphere;

#[cfg(test)]
pub use test_shape::TestShape;
