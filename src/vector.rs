use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};
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

impl<K> Add for Vector<K>
where
    K: Add<Output = K> + Copy,
{
    type Output = Vector<K>;

    fn add(self, rhs: Self) -> Self::Output {
        let elements = self.iter().zip(rhs.iter()).map(|(&a, &b)| a + b).collect();
        Vector { elements }
    }
}

impl<K> Sub for Vector<K>
where
    K: Sub<Output = K> + Copy,
{
    type Output = Vector<K>;

    fn sub(self, rhs: Self) -> Self::Output {
        let elements = self.iter().zip(rhs.iter()).map(|(&a, &b)| a - b).collect();
        Vector { elements }
    }
}

impl<K, T> Mul<T> for Vector<K>
where
    K: Mul<T, Output = K> + Copy,
    T: Copy,
{
    type Output = Vector<K>;

    fn mul(self, rhs: T) -> Self::Output {
        let elements = self.iter().map(|&a| a * rhs).collect();
        Vector { elements }
    }
}

impl<K> Mul for Vector<K>
where
    K: Mul<Output = K> + Copy + Default,
    K: Add<Output = K> + Copy,
{
    type Output = K;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.elements.len() != rhs.elements.len() {
            panic!("The vector need to be on the same plan");
        }

        self.iter().zip(rhs.iter()).map(|(&a, &b)| a * b).fold(K::default(), |acc, x| acc + x)
    }
}

impl<K> Vector<K>
where
    K: Add<Output = K> + Copy + Default,
    K: Sub<Output = K> + Copy + Default,
    K: Mul<Output = K> + Copy + Default,
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
    pub fn dot(&self, v: Self) -> K {
        if self.elements.len() != v.elements.len() {
            panic!("The vector need to be on the same plan");
        }

        self.iter().zip(v.iter()).map(|(&a, &b)| a * b).fold(K::default(), |acc, x| acc + x)
    }
}
