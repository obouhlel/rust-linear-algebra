use super::super::Matrix;
use std::ops::Div;

impl<K, T> Div<T> for Matrix<K>
where
    K: Div<T, Output = K> + Copy + Default,
    T: Copy,
{
    type Output = Matrix<K>;

    fn div(self, rhs: T) -> Self::Output {
        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i][j] = self.elements[i][j] / rhs;
            }
        }

        Matrix { elements }
    }
}

impl<K> Matrix<K>
where
    K: Div<Output = K> + Copy,
{
    pub fn inv_scl(&mut self, a: K) {
        self.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v = *v / a));
    }
}
