use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Vector<K> {
    pub elements: Vec<K>,
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(elements: Vec<K>) -> Self {
        Vector { elements }
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

        self.elements = self.elements.iter().zip(v.elements.iter()).map(|(&a, &b)| a + b).collect();
    }
    pub fn sub(&mut self, v: Vector<K>) {
        if self.elements.len() != v.elements.len() {
            panic!("The vector need to be on the same plan");
        }

        self.elements = self.elements.iter().zip(v.elements.iter()).map(|(&a, &b)| a - b).collect();
    }
    pub fn scl(&mut self, a: K) {
        self.elements = self.elements.iter().map(|&n| n * a).collect();
    }
}
