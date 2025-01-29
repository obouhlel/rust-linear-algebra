use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn matrices_are_equal(a: &Matrix<f64>, b: &Matrix<f64>, epsilon: f64) -> bool {
        if a.rows() != b.rows() || a.cols() != b.cols() {
            return false;
        }
        for i in 0..a.rows() {
            for j in 0..a.cols() {
                if (a.elements[i][j] - b.elements[i][j]).abs() > epsilon {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn inverse_test_1() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let expected = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert!(matrices_are_equal(
            &u.inverse().unwrap(),
            &expected,
            EPSILON
        ));
    }

    #[test]
    fn inverse_test_2() {
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        let expected = Matrix::from([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
        assert!(matrices_are_equal(
            &u.inverse().unwrap(),
            &expected,
            EPSILON
        ));
    }

    #[test]
    fn inverse_test_3() {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        let expected = Matrix::from([
            [0.649425287, 0.097701149, -0.655172414],
            [-0.781609195, -0.126436782, 0.965517241],
            [0.143678161, 0.074712644, -0.206896552],
        ]);
        assert!(matrices_are_equal(
            &u.inverse().unwrap(),
            &expected,
            EPSILON
        ));
    }
}
