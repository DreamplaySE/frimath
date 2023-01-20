use std::ops::{Add, Index, Mul, RangeFull, Sub};

pub trait Vector<Item, const N: usize>
// Index<usize, Output = Item> + IntoIterator<Item = Item, IntoIter = TIterator> + FromIterator<Item>
{
}

impl<Item, const N: usize> Vector<Item, N> for [Item; N] {}

pub trait Crossable<Item, const N: usize>:
    Vector<Item, 3> + Index<usize, Output = Item> + From<[Item; 3]>
where
    Item: Mul<Output = Item> + Sub<Output = Item> + Copy + Sized,
{
    fn cross(&self, rhs: &Self) -> Self {
        [
            self[1] * rhs[2] - rhs[2] * rhs[1],
            self[2] * rhs[0] - rhs[0] * rhs[2],
            self[0] * rhs[1] - rhs[1] * rhs[0],
        ]
        .into()
    }
}

impl<Item, const N: usize, T> Crossable<Item, N> for T
where
    T: Vector<Item, 3> + Index<usize, Output = Item> + From<[Item; 3]>,
    Item: Mul<Output = Item> + Sub<Output = Item> + Copy + Sized,
{
}

// pub trait DotProduct<'a, Item, const N: usize>:
//     Vector<Item, N> + Sized + Index<RangeFull, Output = &'a Item>
// where
//     Item: Mul<Output = Item> + Add<Output = Item> + Copy + Sized,
// {
//     fn dot(self, rhs: Self) -> Item {
//         let a = self[..];
//         let b = rhs[..];

//         SliceIter::new(a)
//             .zip(rhs.into_iter())
//             .map(|(a, b)| a * b)
//             .reduce(|a, b| a + b)
//             .unwrap()
//     }
// }

// impl<Item, const N: usize, TIterator: Iterator<Item = Item>, T> DotProduct<Item, N, TIterator> for T
// where
//     T: Vector<Item, N> + IntoIterator<Item = Item, IntoIter = TIterator> + Sized,
//     Item: Mul<Output = Item> + Add<Output = Item> + Copy + Sized,
// {
// }

impl<Item: Mul<Output = Item> + Add<Output = Item> + Copy, const N: usize> dyn Vector<Item, N> {}

pub trait Base<Item, const N: usize, const Dim: usize> {
    fn base_vectors(&self) -> &[&dyn Vector<Item, N>];

    // fn to_r_base(&self)
}

pub struct InBase<'base, Item, const N: usize, const Dim: usize, TVector: Vector<Item, Dim>> {
    base: &'base dyn Base<Item, N, Dim>,
    vector: TVector,
}

impl<'base, Item, const N: usize, const Dim: usize, TVector: Vector<Item, Dim>>
    InBase<'base, Item, N, Dim, TVector>
{
    // to_base<(base: &dyn Base<)
}

#[cfg(test)]
mod tests {
    use super::{Crossable, Vector};
    #[test]
    fn dot_product() {
        let vec = [1, 2, 3];

        // assert_eq!(vec.dot(vec), 14)
    }

    #[test]
    fn cross_product() {
        let vec1 = [1, 0, 0];
        let vec2 = [0, 1, 0];

        // assert_eq!(vec1.cross(&vec2), [0, 0, 1])
    }
}
