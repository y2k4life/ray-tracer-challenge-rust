//! Contains various shapes used in a scene. The shapes are [`Sphere`] and
//! [`Plane`].
mod cone;
mod cube;
mod cylinder;
mod plane;
mod shape;
mod sphere;
mod test_shape;

pub use cone::Cone;
pub use cube::Cube;
pub use cylinder::Cylinder;
pub use plane::Plane;
pub use shape::Shape;
pub use sphere::Sphere;

#[cfg(test)]
pub use test_shape::TestShape;
