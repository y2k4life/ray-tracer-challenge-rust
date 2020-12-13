use crate::{shapes::Sphere, Point, Vector};

/// Encapsulating precomputed information relating to an [`crate::Intersection`].
pub struct Computations<'a> {
    /// Distance from the origin of a ray to the intersection.
    pub t: f64,
    /// The object intersected by a [`crate::Ray`].
    pub object: &'a Sphere,
    /// Point in world space the intersection occurred.
    pub point: Point,
    /// Adjusted `point` just slightly in the direction of the normal. Bumps the
    /// it above teh surface and prevent self-shadowing.
    pub over_point: Point,
    /// Eye vector pointing back toward the eye or the camera.
    pub eyev: Vector,
    /// Normal vector of the surface of the object intersected.
    pub normalv: Vector,
    /// Intersection occurred inside the shape.
    pub inside: bool,
}
