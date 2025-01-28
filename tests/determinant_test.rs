use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinant_1x1() {
        let u = Matrix::from([[5.0]]);
        assert_eq!(u.determinant(), 5.0);
    }

    #[test]
    fn test_determinant_2x2_zero() {
        let u = Matrix::from([[0.0, 0.0], [0.0, 0.0]]);
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn test_determinant_2x2_identity() {
        let u = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(u.determinant(), 1.0);
    }

    #[test]
    fn test_determinant_3x3_zero_row() {
        let u = Matrix::from([[1.0, 2.0, 3.0], [0.0, 0.0, 0.0], [7.0, 8.0, 9.0]]);
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn test_determinant_3x3_singular() {
        let u = Matrix::from([[2.0, 4.0, 6.0], [1.0, 2.0, 3.0], [3.0, 6.0, 9.0]]);
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn test_determinant_large_4x4() {
        let u = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn test_determinant_4x4_identity() {
        let u = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(u.determinant(), 1.0);
    }

    #[test]
    fn test_determinant_negative_values() {
        let u = Matrix::from([[-2.0, 2.0, -3.0], [-1.0, 1.0, 3.0], [2.0, 0.0, -1.0]]);
        assert_eq!(u.determinant(), 18.0);
    }

    #[test]
    #[should_panic(expected = "Matrix must be square")]
    fn test_determinant_non_square() {
        let u = Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        u.determinant();
    }

    #[test]
    fn test_determinant_large_5x5() {
        let u = Matrix::from([
            [1.0, 2.0, 3.0, 4.0, 5.0],
            [0.0, 2.0, 3.0, 4.0, 5.0],
            [0.0, 0.0, 3.0, 4.0, 5.0],
            [0.0, 0.0, 0.0, 4.0, 5.0],
            [0.0, 0.0, 0.0, 0.0, 5.0],
        ]);
        assert_eq!(u.determinant(), 120.0);
    }

    #[test]
    fn test_determinant_5x5_identity() {
        let u = Matrix::from([
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(u.determinant(), 1.0);
    }

    #[test]
    fn test_determinant_5x5_zero() {
        let u = Matrix::from([
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn test_determinant_5x5_singular() {
        let u = Matrix::from([
            [1.0, 2.0, 3.0, 4.0, 5.0],
            [2.0, 4.0, 6.0, 8.0, 10.0],
            [3.0, 6.0, 9.0, 12.0, 15.0],
            [4.0, 8.0, 12.0, 16.0, 20.0],
            [5.0, 10.0, 15.0, 20.0, 25.0],
        ]);
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn test_determinant_5x5_random() {
        let u = Matrix::from([
            [2.0, 1.0, 3.0, 4.0, 5.0],
            [1.0, 2.0, 3.0, 4.0, 5.0],
            [3.0, 1.0, 2.0, 4.0, 5.0],
            [4.0, 1.0, 3.0, 2.0, 5.0],
            [5.0, 1.0, 3.0, 4.0, 2.0],
        ]);
        assert_eq!(u.determinant(), -90.0);
    }
}
