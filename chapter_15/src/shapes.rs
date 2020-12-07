//! Contains various shapes used in a scene. The shapes are [`Sphere`] and
//! [`Plane`].
mod cone;
mod cube;
mod cylinder;
mod group;
mod plane;
mod shape;
mod sphere;
mod test_shape;
mod triangle;
mod smooth_triangles;

pub use cone::Cone;
pub use cube::Cube;
pub use cylinder::Cylinder;
pub use group::Group;
pub use plane::Plane;
pub use shape::Shape;
pub use sphere::Sphere;
pub use triangle::Triangle;

#[cfg(test)]
pub use test_shape::TestShape;
