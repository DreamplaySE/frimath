use crate::vector::Vector;

use super::Matrix;

impl<const N: usize, const D: usize> Matrix<N, D> {
    pub fn new(vectors: [Vector<N>; D]) -> Self {
        Self { vectors }
    }
}

impl<const N: usize, const D: usize> Default for Matrix<N, D> {
    fn default() -> Self {
        Self {
            vectors: [Vector::<N>::default(); D],
        }
    }
}
