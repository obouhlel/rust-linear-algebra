use super::super::Matrix;
use crate::vector::Vector;
use std::ops::Add;
use std::ops::Mul;

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
