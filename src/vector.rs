use std::ops::{Add, Mul, Sub};
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
        Vector { elements: elements.to_vec() }
    }
}

impl<K> Vector<K> {
    pub fn iter(&self) -> Iter<K> {
        self.elements.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<K> {
        self.elements.iter_mut()
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

impl<K> Vector<K>
where 
    K: Add<Output = K> + Copy, K: Sub<Output = K> + Copy, K: Mul<Output = K> + Copy
{
    pub fn add(&mut self, v: Vector<K>) {
        if self.elements.len() != v.elements.len() {
            panic!("The vector need to be on the same plan");
        }

        self.elements = self.iter().zip(v.iter()).map(|(&a, &b)| a + b).collect();
    }
    pub fn sub(&mut self, v: Vector<K>) {
        if self.elements.len() != v.elements.len() {
            panic!("The vector need to be on the same plan");
        }

        self.elements = self.iter().zip(v.iter()).map(|(&a, &b)| a - b).collect();
    }
    pub fn scl(&mut self, a: K) {
        self.elements = self.iter().map(|&n| n * a).collect();
    }
}