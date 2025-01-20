use std::ops::{ Add, Sub, Mul };
use std::cmp::PartialEq;
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
    Vec<K>: PartialEq, K: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<K> Matrix<K>
where
    K: Add<Output = K> + Copy, K: Sub<Output = K> + Copy, K: Mul<Output = K> + Copy,
{
    pub fn add(&mut self, m: Matrix<K>) {
        if self.elements.len() != m.elements.len() || self.elements[0].len() != m.elements[0].len() {
            panic!("Matrices must have the same dimensions");
        }

        self.iter_mut().zip(m.iter()).for_each(|(row_self, row_other)| {
            row_self.iter_mut().zip(row_other.iter()).for_each(|(a, b)| *a = *a + *b)
        });
    }

    pub fn sub(&mut self, m: Matrix<K>) {
        if self.elements.len() != m.elements.len() || self.elements[0].len() != m.elements[0].len() {
            panic!("Matrices must have the same dimensions");
        }

        self.iter_mut().zip(m.iter()).for_each(|(row_self, row_other)| {
            row_self.iter_mut().zip(row_other.iter()).for_each(|(a, b)| *a = *a - *b)
        });
    }

    pub fn scl(&mut self, a: K) {
        self.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|v| *v = *v * a )
        });
    }
}