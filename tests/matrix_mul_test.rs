use rust_linear_algebra::matrix::Matrix;
use rust_linear_algebra::vector::Vector;

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

    #[test]
    fn matrix_vector_mul_1() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u * v, Vector::from([4., 2.]));
    }

    #[test]
    fn matrix_vector_mul_2() {
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(v), Vector::from([8., 4.]));
    }

    #[test]
    fn matrix_vector_mul_3() {
        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u * v, Vector::from([4., -4.]));
    }

    #[test]
    fn matrix_matrix_mul_1() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.mul_mat(v), Matrix::from([[1., 0.], [0., 1.]]));
    }

    #[test]
    fn matrix_matrix_mul_2() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(v), Matrix::from([[2., 1.], [4., 2.]]));
    }

    #[test]
    fn matrix_matrix_mul_3() {
        let u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(v), Matrix::from([[-14., -7.], [44., 22.]]));
    }

    #[test]
    #[should_panic(expected = "Matrix or vector is empty. Cannot perform multiplication.")]
    fn matrix_vector_mul_empty_matrix() {
        let u: Matrix<f64> = Matrix::from(vec![]);
        let v = Vector::from([4., 2.]);
        let _ = u * v;
    }

    #[test]
    #[should_panic(expected = "Matrix or vector is empty. Cannot perform multiplication.")]
    fn matrix_vector_mul_empty_vector() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v: Vector<f64> = Vector::from(vec![]);
        let _ = u * v;
    }

    #[test]
    #[should_panic(expected = "Incompatible dimensions for matrix-vector multiplication.")]
    fn matrix_vector_mul_incompatible_dimensions() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2., 3.]);
        let _ = u * v;
    }
}
