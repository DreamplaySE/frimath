use crate::vector::Vector;

use super::Matrix;

impl<const N: usize, const D: usize> Matrix<N, D> {
    pub fn new(vectors: [Vector<N>; D]) -> Self {
        Self { vectors }
    }

    pub fn transpose(mut self) -> Matrix::<D, N> {
        let mut output = Matrix::<D, N>::default();

        //Rows
        for (i, vector) in self.vectors.iter().enumerate() {
            //Columns
            for (j, value) in vector.iter().enumerate() {
                *output.vectors[j].get_mut(i).unwrap() = *value;
            }
        }

        output
    }
}

impl<const N: usize, const D: usize> Default for Matrix<N, D> {
    fn default() -> Self {
        Self {
            vectors: [Vector::<N>::default(); D],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn transpose() {
        let a = Matrix::<4, 3>::new([
            Vector::new([1.0, 2.0, 3.0, 4.0]),
            Vector::new([5.0, 6.0, 7.0, 8.0]),
            Vector::new([9.0, 10.0, 11.0, 12.0]),
        ]);
        let b = Matrix::<3, 4>::new([
            Vector::new([1.0, 5.0, 9.0]),
            Vector::new([2.0, 6.0, 10.0]),
            Vector::new([3.0, 7.0, 11.0]),
            Vector::new([4.0, 8.0, 12.0]),
        ]);

        assert_eq!(a.transpose(), b);
        assert_eq!(a, b.transpose());
    }
}