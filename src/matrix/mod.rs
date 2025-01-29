pub mod determinant;
mod gaussian_elemination;
pub mod identity;
pub mod inverse;
pub mod ops;
pub mod row_echelon;
pub mod trace;
pub mod transpose;

pub use ops::*;

use std::cmp::PartialEq;
use std::fmt::Debug;
use std::slice::{Iter, IterMut};
use std::usize;

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

impl<K: Default + Clone> Matrix<K> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let elements = vec![vec![K::default(); cols]; rows];
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
