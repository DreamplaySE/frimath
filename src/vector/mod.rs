pub mod generic;
mod vector2d;
pub use vector2d::Vector2D;
mod vector3d;
pub use vector3d::Vector3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    values: [f64; N],
}
