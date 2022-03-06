use super::Vector;

pub type Vector2D = Vector<2>;

impl Vector2D {
    pub fn new_2d(x: f64, y: f64) -> Self {
        Vector2D { values: [x, y] }
    }

    pub fn x(&self) -> &f64 {
        &self.values[0]
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.values[0]
    }

    pub fn y(&self) -> &f64 {
        &self.values[1]
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.values[1]
    }

    // pub fn intersect_point_in_polygon<const K: usize>(
    //   &self,
    //   polygon: &[Self; K],
    // ) -> bool {
    //   let search_line = [*self, Vector::new([*self.x(), f64::MAX])];
    //   let mut is_in = false;

    //   for i in 0..K {
    //       let a = polygon[i];
    //       let b = polygon[i+1 % K];

    //       let line = [a, b];

    //       if Vector::intersect_segments(&search_line, &line).is_some() {
    //         is_in = !is_in;
    //       }
    //   }

    //   return is_in;
    // }
}
