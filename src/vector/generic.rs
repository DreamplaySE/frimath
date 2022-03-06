use std::{
    ops::{Add, Div, Mul, Sub},
    slice::{Iter, IterMut},
};

use super::Vector;

impl<const N: usize> Vector<N> {
    pub fn new(values: [f64; N]) -> Self {
        Vector { values }
    }

    pub fn empty() -> Self {
        Vector::default()
    }

    pub fn get(&self, index: usize) -> Option<&f64> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.values.get_mut(index)
    }

    pub fn dot_product(&self, target: &Self) -> f64 {
        let mut total = 0.0;

        for i in 0..N {
            total += self.values[i] * target.values[i];
        }

        total
    }

    pub fn normalize(mut self) -> Self {
        let magnitude = self.magnitude();

        for value in self.iter_mut() {
            *value /= magnitude;
        }

        self
    }

    pub fn magnitude(&self) -> f64 {
        let mut total = 0.0;

        for i in 0..N {
            total += self.values[i] * self.values[i];
        }

        total.sqrt()
    }

    pub fn values(&self) -> &[f64; N] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [f64; N] {
        &mut self.values
    }

    pub fn iter(&self) -> Iter<'_, f64> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, f64> {
        self.values.iter_mut()
    }

    pub fn round(mut self) -> Self {
        for value in self.iter_mut() {
            *value = value.round();
        }

        self
    }

    pub fn round_to_point(mut self, multitude: f64) -> Self {
        for value in self.iter_mut() {
            *value = (*value * multitude).round() / multitude
        }

        self
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = [f64::default(); N];

        for i in 0..N {
            values[i] = self.values[i] + rhs.values[i];
        }

        Vector { values }
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut values = [f64::default(); N];

        for i in 0..N {
            values[i] = self.values[i] - rhs.values[i];
        }

        Vector { values }
    }
}

impl<const N: usize> Mul for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = [f64::default(); N];

        for i in 0..N {
            values[i] = self.values[i] * rhs.values[i];
        }

        Vector { values }
    }
}

impl<const N: usize> Div for Vector<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut values = [f64::default(); N];

        for i in 0..N {
            values[i] = self.values[i] / rhs.values[i];
        }

        Vector { values }
    }
}

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut values = [f64::default(); N];

        for i in 0..N {
            values[i] = self.values[i] * rhs;
        }

        Vector { values }
    }
}

impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut values = [f64::default(); N];

        for i in 0..N {
            values[i] = self.values[i] / rhs;
        }

        Vector { values }
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self { values: [0.0; N] }
    }
}
