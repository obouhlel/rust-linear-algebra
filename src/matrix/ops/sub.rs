use super::super::Matrix;
use std::ops::Sub;

impl<K> Sub for Matrix<K>
where
    K: Sub<Output = K> + Copy + Default,
{
    type Output = Matrix<K>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            panic!("Matrices must have the same dimensions");
        }

        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i][j] = self.elements[i][j] - rhs.elements[i][j];
            }
        }

        Matrix { elements }
    }
}

impl<K> Matrix<K>
where
    K: Sub<Output = K> + Copy,
{
    pub fn sub(&mut self, m: Matrix<K>) {
        if self.rows() != m.rows() || self.cols() != m.cols() {
            panic!("Matrices must have the same dimensions");
        }

        self.iter_mut()
            .zip(m.iter())
            .for_each(|(row_self, row_other)| {
                row_self
                    .iter_mut()
                    .zip(row_other.iter())
                    .for_each(|(a, b)| *a = *a - *b)
            });
    }
}
