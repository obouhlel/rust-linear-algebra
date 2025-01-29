use super::Matrix;
use crate::one::One;

pub fn identity_mat<K>(cols: usize, rows: usize) -> Matrix<K>
where
    K: Copy + Default + One,
{
    let mut elements = vec![vec![K::default(); cols]; rows];

    let min_dim = usize::min(cols, rows);
    for i in 0..min_dim {
        elements[i][i] = K::one();
    }

    Matrix { elements }
}
