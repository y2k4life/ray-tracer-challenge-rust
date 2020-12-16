#[allow(unused_imports)]
use crate::Intersection;
use crate::{shapes::Shape, Point, Vector};

/// Encapsulating precomputed information relating to an [`Intersection`].
pub struct Computations<'a> {
    /// Distance from the origin of a ray to the intersection.
    pub t: f64,
    /// The object intersected by a [`crate::Ray`].
    pub object: &'a dyn Shape,
    /// Point in world space the intersection occurred.
    pub point: Point,
    /// Adjusted `point` just slightly in the direction of the normal. Bumps
    /// `point` above the surface and prevents self-shadowing.
    pub over_point: Point,
    /// Adjusted `point` just slightly under in the direction of the normal. Bumps the
    /// it above teh surface and prevent self-shadowing.
    pub under_point: Point,
    /// Eye vector pointing back toward the eye or the camera.
    pub eyev: Vector,
    /// Normal vector of the surface of the object intersected.
    pub normalv: Vector,
    /// Intersection occurred inside the shape.
    pub inside: bool,
    /// A rays reflective vector
    pub reflectv: Vector,
    /// The distance from the origin of a refractive ray to the point it
    /// exits a material
    pub n1: f64,
    /// The distance from the origin of a refractive ray to the point it
    /// enters a material
    pub n2: f64,
}

impl Computations<'_> {
    pub fn schlick(&self) -> f64 {
        // find the cosine of the angle between the eye and normal vector
        let mut cos = self.eyev.dot(self.normalv);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            // computer cosine of theta_t using trig identity
            // when n1 > n2 use cos(theta_t) instead
            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
