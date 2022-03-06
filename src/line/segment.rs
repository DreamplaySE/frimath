use crate::vector::Vector;

use super::Line;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment<const N: usize> {
    start: Vector<N>,
    end: Vector<N>,
}

pub enum Intersection<const N: usize> {
    Segment(Line<N>),
    Point(Vector<N>),
    OutOfBounds(Vector<N>),
    None,
}

impl<const N: usize> Segment<N> {
    pub fn new(start: Vector<N>, end: Vector<N>) -> Self {
        Segment { start, end }
    }

    pub fn line(&self) -> Line<N> {
        Line::new(self.end - self.start, self.start)
    }

    pub fn intersect(&self) -> Intersection<N> {
        let intersection = self.line();

        Intersection::Point(self.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product_test() {
        let a = Vector::new([3.0, -3.0, 1.0]);
        let b = Vector::new([4.0, 9.0, 2.0]);

        assert_eq!(a.cross_product(&b), Vector::new([-15.0, -2.0, 39.0]));
    }
}
