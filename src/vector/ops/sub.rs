use super::super::Vector;
use std::ops::Sub;

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

impl<K> Vector<K>
where
    K: Sub<Output = K> + Copy + Default,
{
    pub fn sub(&mut self, v: Vector<K>) {
        if self.len() != v.len() {
            panic!("The vector need to be on the same plan");
        }

        self.elements = self.iter().zip(v.iter()).map(|(&a, &b)| a - b).collect();
    }
}