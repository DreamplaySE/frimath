use crate::vector::Vector;
pub mod generic;

struct Matrix<const N: usize, const D: usize> {
    vectors: [Vector<N>; D],
}
