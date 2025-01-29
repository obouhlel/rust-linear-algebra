use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank() {
        let u = Matrix::from([[1., 0., 1.], [0., 2., 1.], [1., 1., 1.]]);

        assert_eq!(u.rank(), 3);
    }

    #[test]
    fn test_rank_identity_matrix() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(u.rank(), 3);
    }

    #[test]
    fn test_rank_dependent_rows() {
        let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
        assert_eq!(u.rank(), 2);
    }

    #[test]
    fn test_rank_rectangular_matrix() {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
        assert_eq!(u.rank(), 3);
    }
}
