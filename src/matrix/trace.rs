use super::Matrix;
use std::ops::Add;

impl<K> Matrix<K>
where
    K: Copy + Default + Add<Output = K>,
{
    pub fn trace(&self) -> K {
        if self.cols() != self.rows() {
            panic!("Matrix must be a square");
        }

        (1..self.rows()).fold(self.elements[0][0], |acc, i| acc + self.elements[i][i])
    }
}
