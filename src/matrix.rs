use crate::vector::Vector;
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};
use std::slice::{Iter, IterMut};

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub elements: Vec<Vec<K>>,
}

impl<K> From<Vec<Vec<K>>> for Matrix<K> {
    fn from(elements: Vec<Vec<K>>) -> Self {
        Matrix { elements }
    }
}

impl<K, const ROWS: usize, const COLS: usize> From<[[K; COLS]; ROWS]> for Matrix<K>
where
    K: Clone,
{
    fn from(array: [[K; COLS]; ROWS]) -> Self {
        let elements = array.iter().map(|row| row.to_vec()).collect();
        Matrix { elements }
    }
}

impl<K> Matrix<K> {
    pub fn iter(&self) -> Iter<Vec<K>> {
        self.elements.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<Vec<K>> {
        self.elements.iter_mut()
    }
}

impl<K> PartialEq for Matrix<K>
where
    Vec<K>: PartialEq,
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<K> Add for Matrix<K>
where
    K: Add<Output = K> + Copy + Default,
{
    type Output = Matrix<K>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.elements.len() != rhs.elements.len()
            || self.elements[0].len() != rhs.elements[0].len()
        {
            panic!("Matrices must have the same dimensions");
        }

        let mut elements = vec![vec![K::default(); self.elements[0].len()]; self.elements.len()];

        for i in 0..self.elements.len() {
            for j in 0..self.elements[i].len() {
                elements[i][j] = self.elements[i][j] + rhs.elements[i][j];
            }
        }

        Matrix { elements }
    }
}

impl<K> Sub for Matrix<K>
where
    K: Sub<Output = K> + Copy + Default,
{
    type Output = Matrix<K>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.elements.len() != rhs.elements.len()
            || self.elements[0].len() != rhs.elements[0].len()
        {
            panic!("Matrices must have the same dimensions");
        }

        let mut elements = vec![vec![K::default(); self.elements[0].len()]; self.elements.len()];

        for i in 0..self.elements.len() {
            for j in 0..self.elements[i].len() {
                elements[i][j] = self.elements[i][j] - rhs.elements[i][j];
            }
        }

        Matrix { elements }
    }
}

impl<K, T> Mul<T> for Matrix<K>
where
    K: Mul<T, Output = K> + Copy + Default,
    T: Copy,
{
    type Output = Matrix<K>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut elements = vec![vec![K::default(); self.elements[0].len()]; self.elements.len()];

        for i in 0..self.elements.len() {
            for j in 0..self.elements[0].len() {
                elements[i][j] = self.elements[i][j] * rhs;
            }
        }

        Matrix { elements }
    }
}

impl<K> Mul for Matrix<K>
where
    K: Mul<Output = K> + Add<Output = K> + Copy + Default,
{
    type Output = Matrix<K>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.elements.is_empty() || rhs.elements.is_empty() {
            panic!("One of the matrices is empty. Cannot perform multiplication.");
        }
        if self.elements[0].len() != rhs.elements.len() {
            panic!(
                "Incompatible dimensions for matrix multiplication. \
                Left matrix has {} columns, but right matrix has {} rows.",
                self.elements[0].len(),
                rhs.elements.len()
            );
        }

        let mut elements = vec![vec![K::default(); rhs.elements[0].len()]; self.elements.len()];

        for i in 0..self.elements.len() {
            for j in 0..rhs.elements[0].len() {
                for k in 0..self.elements[0].len() {
                    elements[i][j] = elements[i][j] + (self.elements[i][k] * rhs.elements[k][j]);
                }
            }
        }

        Matrix { elements }
    }
}

impl<K> Matrix<K>
where
    K: Add<Output = K> + Copy,
{
    pub fn add(&mut self, m: Matrix<K>) {
        if self.elements.len() != m.elements.len() || self.elements[0].len() != m.elements[0].len()
        {
            panic!("Matrices must have the same dimensions");
        }

        self.iter_mut()
            .zip(m.iter())
            .for_each(|(row_self, row_other)| {
                row_self
                    .iter_mut()
                    .zip(row_other.iter())
                    .for_each(|(a, b)| *a = *a + *b)
            });
    }
}

impl<K> Matrix<K>
where
    K: Sub<Output = K> + Copy,
{
    pub fn sub(&mut self, m: Matrix<K>) {
        if self.elements.len() != m.elements.len() || self.elements[0].len() != m.elements[0].len()
        {
            panic!("Matrices must have the same dimensions");
        }

        self.iter_mut()
            .zip(m.iter())
            .for_each(|(row_self, row_other)| {
                row_self
                    .iter_mut()
                    .zip(row_other.iter())
                    .for_each(|(a, b)| *a = *a - *b)
            });
    }
}

impl<K> Matrix<K>
where
    K: Mul<Output = K> + Copy,
{
    pub fn scl(&mut self, a: K) {
        self.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v = *v * a));
    }
}

impl<K> Matrix<K>
where
    K: Mul<Output = K> + Add<Output = K> + Copy + Default,
{
    pub fn mul_mat(&self, mat: Matrix<K>) -> Matrix<K> {
        self.clone() * mat
    }
}
