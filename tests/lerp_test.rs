use rust_linear_algebra::matrix::Matrix;
use rust_linear_algebra::utils::lerp;
use rust_linear_algebra::vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let start = 0.0;
        let end = 10.0;
        let t = 0.5;
        let result = lerp(start, end, t);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_case_2() {
        let start = 0.0;
        let end = 1.0;
        let t = 1.0;
        let result = lerp(start, end, t);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_case_3() {
        let start = 0.0;
        let end = 1.0;
        let t = 0.5;
        let result = lerp(start, end, t);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_case_4() {
        let start = 21.0;
        let end = 42.0;
        let t = 0.3;
        let result = lerp(start, end, t);
        assert_eq!(result, 27.3);
    }

    #[test]
    fn test_vector_lerp() {
        let start = Vector::from([2.0, 1.0]);
        let end = Vector::from([4.0, 2.0]);
        let t = 0.3;
        let result = lerp(start, end, t);
        assert_eq!(result, Vector::from([2.6, 1.3]));
    }

    #[test]
    fn test_matrix_lerp() {
        let start = Matrix::from([[2.0, 1.0], [3.0, 4.0]]);
        let end = Matrix::from([[20.0, 10.0], [30.0, 40.0]]);
        let t = 0.5;
        let result = lerp(start, end, t);
        assert_eq!(result, Matrix::from([[11.0, 5.5], [16.5, 22.0]]));
    }

    #[test]
    #[should_panic]
    fn test_panic_1() {
        let start = 10.0;
        let end = 20.0;
        let t = -1.0;
        lerp(start, end, t);
    }

    #[test]
    #[should_panic]
    fn test_panic_2() {
        let start = 10.0;
        let end = 20.0;
        let t = 39.0;
        lerp(start, end, t);
    }
}
