use super::super::Vector;
use std::ops::Add;

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

impl<K> Vector<K>
where
    K: Add<Output = K> + Copy + Default,
{
    pub fn add(&mut self, v: Vector<K>) {
        if self.len() != v.len() {
            panic!("The vector need to be on the same plan");
        }

        self.elements = self.iter().zip(v.iter()).map(|(&a, &b)| a + b).collect();
    }
}