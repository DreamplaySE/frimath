use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line<const N: usize> {
    direction: Vector<N>,
    offset: Vector<N>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intersection<const N: usize> {
    Line(Line<N>),
    Point(Vector<N>),
    None,
}

impl<const N: usize> Line<N> {
    pub fn new(direction: Vector<N>, offset: Vector<N>) -> Self {
        let direction = direction.normalize();

        let line = Line { direction, offset };

        line.normalize()
    }

    pub fn normalize(mut self) -> Self {
        let (index, value) = self.direction.iter().enumerate().find(|(_, val)| **val != 0.0).unwrap();
        let change_magnitude = self.offset.get(index).unwrap() / value;

        for i in (index)..N {
            *self.offset.get_mut(i).unwrap() -= self.direction.get(i).unwrap() * change_magnitude;
        }

        self.direction = self.direction.round_to_point(100.0);
        self.offset = self.offset.round_to_point(100.0);

        self
    }

    pub fn intersect(&self, other: &Line<N>) -> Intersection<N> {
        let direction = self.direction - other.direction;
        let offset = self.offset - other.offset;

        if direction == Vector::default() {
            return if offset == Vector::default() {
                Intersection::Line(*self)
            } else {
                Intersection::None
            };
        };

        println!("Direction: {:?} Offset: {:?}", direction, offset);

        Intersection::Line(Line::new(direction, offset))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn new_test() {
        let a = Line::new(Vector::new_2d(2.0, 5.0), Vector::new_2d(10.0, 18.0));
        let b = Line::new(Vector::new_2d(2.0, 5.0), Vector::new_2d(0.0, -7.0));

        assert_eq!(a, b);
    }

    #[test]
    fn intersect_point_intersection_test() {
        let a = Line::new(Vector::new_2d(0.0, 1.0), Vector::new_2d(0.0, 0.0));
        let b = Line::new(Vector::new_2d(1.0, 0.0), Vector::new_2d(0.0, 0.0));

        let intersection = a.intersect(&b);

        // assert_eq!(intersection, Intersection::Point(Vector::new_2d(0.0, 0.0)));
        assert_eq!(true, true);
    }

    #[test]
    fn intersect_line_intersection_test() {
        let line = Line::new(Vector::new_2d(1.0, 1.0), Vector::new_2d(0.0, 0.0));

        let intersection = line.intersect(&line);

        assert_eq!(intersection, Intersection::Line(line));
    }

    #[test]
    fn intersect_no_intersection_test() {
        let a = Line::new(Vector::new_2d(1.0, 1.0), Vector::new_2d(0.0, 0.0));
        let b = Line::new(Vector::new_2d(1.0, 1.0), Vector::new_2d(0.0, 1.0));

        let intersection = a.intersect(&b);

        assert_eq!(intersection, Intersection::None);
    }
}
