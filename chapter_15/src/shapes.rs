//! Contains various shapes used in a scene. The shapes are [`Sphere`] and
//! [`Plane`].
macro_rules! impl_shape_common {
    () => {
        fn id(&self) -> u64 {
            self.id
        }

        fn parent_id(&self) -> Option<u64> {
            self.parent_id
        }

        fn set_parent_id(&mut self, id: u64) {
            self.parent_id = Some(id);
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

mod cone;
mod cube;
mod cylinder;
mod group;
mod plane;
mod shape;
mod smooth_triangles;
mod sphere;
mod test_shape;
mod triangle;

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
