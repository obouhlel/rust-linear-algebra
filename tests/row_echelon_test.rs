use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matrix_eq_with_tolerance(
        matrix1: &Matrix<f64>,
        matrix2: &Matrix<f64>,
        tolerance: f64,
    ) {
        assert_eq!(matrix1.rows(), matrix2.rows());
        assert_eq!(matrix1.cols(), matrix2.cols());
        for i in 0..matrix1.rows() {
            for j in 0..matrix1.cols() {
                assert!(
                    (matrix1.elements[i][j] - matrix2.elements[i][j]).abs() < tolerance,
                    "Matrices are not equal within tolerance"
                );
            }
        }
    }

    // Test 1: Matrice inversible
    #[test]
    fn test_row_echelon_invertible() {
        let matrix = Matrix::from(vec![vec![2, 1, -1], vec![-3, -1, 2], vec![-2, 1, 2]]);
        let expected = Matrix::from(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 2: Matrice non inversible
    #[test]
    fn test_row_echelon_non_invertible() {
        let matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let expected = Matrix::from(vec![vec![1, 0, -1], vec![0, 1, 2], vec![0, 0, 0]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 3: Matrice avec des lignes linéairement dépendantes
    #[test]
    fn test_row_echelon_linearly_dependent() {
        let matrix = Matrix::from(vec![vec![1, 2, 1], vec![2, 4, 2], vec![3, 6, 3]]);
        let expected = Matrix::from(vec![vec![1, 2, 1], vec![0, 0, 0], vec![0, 0, 0]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 4: Matrice vide
    #[test]
    fn test_row_echelon_empty() {
        let matrix = Matrix::<i32>::from(vec![]);
        let expected = Matrix::from(vec![]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 5: Matrice avec une seule ligne
    #[test]
    fn test_row_echelon_single_row() {
        let matrix = Matrix::from(vec![vec![1, 2, 3]]);
        let expected = Matrix::from(vec![vec![1, 2, 3]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 6: Matrice avec une seule colonne
    #[test]
    fn test_row_echelon_single_column() {
        let matrix = Matrix::from(vec![vec![1], vec![2], vec![3]]);
        let expected = Matrix::from(vec![vec![1], vec![0], vec![0]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 7: Matrice avec des zéros partout
    #[test]
    fn test_row_echelon_all_zeros() {
        let matrix = Matrix::from(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
        let expected = Matrix::from(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 8: Matrice rectangulaire (plus de lignes que de colonnes)
    #[test]
    fn test_row_echelon_rectangular_more_rows() {
        let matrix = Matrix::from(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
        let expected = Matrix::from(vec![vec![1, 0], vec![0, 1], vec![0, 0]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 9: Matrice rectangulaire (plus de colonnes que de lignes)
    #[test]
    fn test_row_echelon_rectangular_more_columns() {
        let matrix = Matrix::from(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
        let expected = Matrix::from(vec![vec![1, 0, -1, -2], vec![0, 1, 2, 3]]);
        assert_eq!(matrix.row_echelon(), expected);
    }

    // Test 10: Matrice identité
    #[test]
    fn test_row_echelon_identity() {
        let matrix = Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]);
        let expected = Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]);
        assert_matrix_eq_with_tolerance(&matrix.row_echelon(), &expected, 1e-6);
    }

    // Test 11: Matrice 2x2
    #[test]
    fn test_row_echelon_2x2() {
        let matrix = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
        let expected = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
        assert_matrix_eq_with_tolerance(&matrix.row_echelon(), &expected, 1e-6);
    }

    // Test 12: Matrice 2x2 avec lignes dépendantes
    #[test]
    fn test_row_echelon_2x2_dependent() {
        let matrix = Matrix::from(vec![vec![1., 2.], vec![2., 4.]]);
        let expected = Matrix::from(vec![vec![1., 2.], vec![0., 0.]]);
        assert_matrix_eq_with_tolerance(&matrix.row_echelon(), &expected, 1e-6);
    }

    // Test 13: Matrice 3x5
    #[test]
    fn test_row_echelon_3x5() {
        let matrix = Matrix::from(vec![
            vec![8., 5., -2., 4., 28.],
            vec![4., 2.5, 20., 4., -4.],
            vec![8., 5., 1., 4., 17.],
        ]);
        let expected = Matrix::from(vec![
            vec![1., 0.625, 0.0, 0.0, -12.1666667],
            vec![0., 0.0, 1.0, 0.0, -3.6666667],
            vec![0., 0.0, 0.0, 1.0, 29.5],
        ]);
        assert_matrix_eq_with_tolerance(&matrix.row_echelon(), &expected, 1e-6);
    }
}
