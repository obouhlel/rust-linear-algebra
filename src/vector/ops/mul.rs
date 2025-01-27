use super::super::Vector;
use std::ops::{Mul, Add};

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

impl<K> Vector<K>
where
    K: Mul<Output = K> + Copy + Default,
{
    pub fn scl(&mut self, a: K) {
        self.elements = self.iter().map(|&n| n * a).collect();
    }
}

impl<K> Mul for Vector<K>
where
    K: Mul<Output = K> + Copy + Default,
    K: Add<Output = K> + Copy,
{
    type Output = K;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("The vector need to be on the same plan");
        }

        self.iter()
            .zip(rhs.iter())
            .map(|(&a, &b)| a * b)
            .fold(K::default(), |acc, x| acc + x)
    }
}

impl<K> Vector<K>
where
    K: Add<Output = K> + Copy + Default,
    K: Mul<Output = K> + Copy + Default,
{
    pub fn dot(&self, v: Self) -> K {
        if self.len() != v.len() {
            panic!("The vector need to be on the same plan");
        }

        self.iter()
            .zip(v.iter())
            .map(|(&a, &b)| a * b)
            .fold(K::default(), |acc, x| acc + x)
    }
}