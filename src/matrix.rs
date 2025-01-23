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

impl<K> Matrix<K> {
    pub fn new(elements: Vec<Vec<K>>) -> Self {
        Matrix { elements }
    }
}

impl<K> Matrix<K> {
    pub fn rows(&self) -> usize {
        self.elements.len()
    }
    pub fn cols(&self) -> usize {
        self.elements[0].len()
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
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            panic!("Matrices must have the same dimensions");
        }

        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
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
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            panic!("Matrices must have the same dimensions");
        }

        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
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
        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i][j] = self.elements[i][j] * rhs;
            }
        }

        Matrix { elements }
    }
}

impl<K> Mul<Vector<K>> for Matrix<K>
where
    K: Copy + Default + Add<Output = K> + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, rhs: Vector<K>) -> Self::Output {
        if self.elements.is_empty() || rhs.elements.is_empty() {
            panic!("Matrix or vector is empty. Cannot perform multiplication.");
        }
        if self.cols() != rhs.len() {
            panic!(
                "Incompatible dimensions for matrix-vector multiplication. \
                Matrix has {} columns, but vector has {} elements.",
                self.cols(),
                rhs.len()
            );
        }

        let mut elements = vec![K::default(); self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i] = elements[i] + self.elements[i][j] * rhs.elements[j];
            }
        }

        Vector { elements }
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
        if self.cols() != rhs.rows() {
            panic!(
                "Incompatible dimensions for matrix multiplication. \
                Left matrix has {} columns, but right matrix has {} rows.",
                self.cols(),
                rhs.rows()
            );
        }

        let mut elements = vec![vec![K::default(); rhs.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                for k in 0..self.cols() {
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
        if self.rows() != m.rows() || self.cols() != m.cols() {
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
        if self.rows() != m.rows() || self.cols() != m.cols() {
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
    pub fn mul_vec(&self, vec: Vector<K>) -> Vector<K> {
        self.clone() * vec
    }
}

impl<K> Matrix<K>
where
    K: Copy + Add<Output = K>,
{
    pub fn trace(&self) -> K {
        if self.cols() != self.rows() {
            panic!("Matrix must be a square");
        }

        let mut sum = self.elements[0][0];
        for i in 1..self.rows() {
            sum = sum + self.elements[i][i];
        }
        sum
    }
}

impl<K> Matrix<K>
where
    K: Copy + Default,
{
    pub fn transpose(&self) -> Matrix<K> {
        let mut elements = vec![vec![K::default(); self.rows()]; self.cols()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[j][i] = self.elements[i][j];
            }
        }

        Matrix { elements }
    }
}
