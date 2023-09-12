use std::ops::{Add, Div, Index, Mul, Sub};

pub trait VectorItem
where
    Self: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Copy
        + Sized,
{
}

impl<T> VectorItem for T where
    T: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Copy
        + Sized
{
}

pub trait Vector<'a, Item: VectorItem + 'a, const N: usize>:
    Index<usize, Output = Item> + Sized
{
    fn dot(&self, rhs: &Self) -> Item {
        let mut res: Option<Item> = None;

        for i in 0..N {
            let local_res: Item = self[i] * rhs[i];

            res = Some(match res {
                Some(res) => res + local_res,
                None => local_res,
            });
        }

        res.unwrap()
    }
}

impl<'a, Item: VectorItem + 'a, const N: usize> Vector<'a, Item, N> for [Item; N] {}

pub trait Crossable<'a, Item: VectorItem + 'a>: Vector<'a, Item, 3> + From<[Item; 3]> {
    fn cross(&self, rhs: &Self) -> Self {
        [
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ]
        .into()
    }
}

impl<'a, Item: VectorItem + 'a, T: Vector<'a, Item, 3> + From<[Item; 3]>> Crossable<'a, Item>
    for T
{
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dot_product() {
        let vec = [1, 2, 3];
        // vec.iter()

        assert_eq!(vec.dot(&vec), 14)
    }

    #[test]
    fn cross_product() {
        let vec1 = [1, 0, 0];
        let vec2 = [0, 1, 0];

        assert_eq!(vec1.cross(&vec2), [0, 0, 1])
    }
}
