use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn matrix_mul() {
            let m1 = Matrix::from([[1, 4], [3, 5]]);

            let m2 = Matrix::from([[2, 5], [5, 1]]);

            assert_eq!(m1 * m2, Matrix::from([[22, 9], [31, 20]]));
        }

        #[test]
        fn matrix_mul_identity() {
            let m1 = Matrix::from([[1, 2], [3, 4]]);

            let identity = Matrix::from([[1, 0], [0, 1]]);

            assert_eq!(m1.clone() * identity, m1);
        }

        #[test]
        fn matrix_mul_zero() {
            let m1 = Matrix::from([[1, 2], [3, 4]]);

            let zero = Matrix::from([[0, 0], [0, 0]]);

            assert_eq!(m1 * zero.clone(), zero);
        }

        #[test]
        fn matrix_mul_different_sizes() {
            let m1 = Matrix::from([[1, 2, 3], [4, 5, 6]]);

            let m2 = Matrix::from([[7, 8], [9, 10], [11, 12]]);

            assert_eq!(m1 * m2, Matrix::from([[58, 64], [139, 154]]));
        }
    }
}
