//! Contains various shapes used in a scene. The shapes are [`Sphere`] and
//! [`Plane`].
macro_rules! impl_shape_common {
    () => {
        fn id(&self) -> u64 {
            self.id
        }

        fn transform(&self) -> Matrix {
            self.transform
        }

        fn set_transform(&mut self, transform: Matrix) {
            self.transform = transform;
        }

        fn material(&self) -> &Material {
            &self.material
        }

        fn material_mut(&mut self) -> &mut Material {
            &mut self.material
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }
    };
}

mod plane;
mod shape;
mod sphere;
mod test_shape;

pub use plane::Plane;
pub use shape::Shape;
pub use sphere::Sphere;

#[cfg(test)]
pub use test_shape::TestShape;
