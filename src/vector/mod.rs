pub mod ops;
pub mod angle_cos;
pub mod cross_product;
pub mod linear_combinaison;
pub mod norm;

use std::cmp::PartialEq;
use std::slice::{Iter, IterMut};

#[derive(Debug, Clone)]
pub struct Vector<K> {
    pub elements: Vec<K>,
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(elements: Vec<K>) -> Self {
        Vector { elements }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K>
where
    K: Clone,
{
    fn from(elements: [K; N]) -> Self {
        Vector {
            elements: elements.to_vec(),
        }
    }
}

impl<K> Vector<K> {
    pub fn new(elements: Vec<K>) -> Self {
        Vector { elements }
    }
    pub fn iter(&self) -> Iter<K> {
        self.elements.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<K> {
        self.elements.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<K> PartialEq for Vector<K>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
    fn ne(&self, other: &Self) -> bool {
        self.elements != other.elements
    }
}
