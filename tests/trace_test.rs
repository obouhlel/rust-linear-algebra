use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_2x2_identity() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.trace(), 2.0);
    }

    #[test]
    fn test_trace_3x3() {
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(u.trace(), 9.0);
    }

    #[test]
    fn test_trace_3x3_negative() {
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(u.trace(), -21.0);
    }

    #[test]
    #[should_panic(expected = "Matrix must be a square")]
    fn test_trace_panic() {
        let u = Matrix::from([[1., 0.], [0., 1.], [0., 0.]]);
        u.trace();
    }
}
