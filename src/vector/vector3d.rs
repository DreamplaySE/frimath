use super::Vector;

pub type Vector3D = Vector<3>;

impl Vector3D {
    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        Vector3D { values: [x, y, z] }
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

    pub fn z(&self) -> &f64 {
        &self.values[2]
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.values[2]
    }

    pub fn cross_product(&self, target: &Self) -> Self {
        let values = [
            self.values[1] * target.values[2] - self.values[2] * target.values[1],
            self.values[2] * target.values[0] - self.values[0] * target.values[2],
            self.values[0] * target.values[1] - self.values[1] * target.values[0],
        ];

        Vector { values }
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
