use super::Matrix;

impl<K> Matrix<K>
where
    K: Copy + Default,
{
    pub fn transpose(&self) -> Matrix<K> {
        let mut elements = vec![vec![K::default(); self.rows()]; self.cols()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[j][i] = self.elements[i][j];
            }
        }

        Matrix { elements }
    }
}
