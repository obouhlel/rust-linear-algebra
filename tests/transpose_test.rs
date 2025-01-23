use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_test_1() {
        let matrix = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let transposed = matrix.transpose();
        let expected = Matrix::new(vec![vec![1, 3], vec![2, 4]]);
        assert_eq!(transposed, expected);
    }

    #[test]
    fn transpose_test_2() {
        let matrix = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let transposed = matrix.transpose();
        let expected = Matrix::new(vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
        assert_eq!(transposed, expected);
    }

    #[test]
    fn transpose_test_3() {
        let matrix = Matrix::new(vec![vec![1]]);
        let transposed = matrix.transpose();
        let expected = Matrix::new(vec![vec![1]]);
        assert_eq!(transposed, expected);
    }
}
