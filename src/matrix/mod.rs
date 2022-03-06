use crate::vector::Vector;
pub mod generic;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const N: usize, const D: usize> {
    vectors: [Vector<N>; D],
}
