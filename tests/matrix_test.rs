use rust_linear_algebra::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut m1 = Matrix::from([
            [1, 3],
            [4, 2]
        ]);
        let m2 = Matrix::from([
            [4, 1],
            [2, 3]
        ]);

        m1.add(m2);
        assert_eq!(m1, Matrix::from([[5,4],[6,5]]));
    }

    #[test]
    #[should_panic]
    fn add_panic() {
        let mut m1 = Matrix::from([
            [1, 3, 2],
            [4, 2, 2]
        ]);
        let m2 = Matrix::from([
            [4, 1],
            [2, 3]
        ]);

        m1.add(m2);
    }

    #[test]
    fn sub() {
        let mut m1 = Matrix::from([
            [5, 4],
            [6, 5]
        ]);
        let m2 = Matrix::from([
            [4, 1],
            [2, 3]
        ]);

        m1.sub(m2);
        assert_eq!(m1, Matrix::from([[1, 3],[4, 2]]));
    }

    #[test]
    #[should_panic]
    fn sub_panic() {
        let mut m1 = Matrix::from([
            [1, 3, 2],
            [4, 2, 2]
        ]);
        let m2 = Matrix::from([
            [4, 1],
            [2, 3]
        ]);

        m1.sub(m2);
    }

    #[test]
    fn scl() {
        let mut m = Matrix::from([
            [1, 2],
            [3, 4]
        ]);

        m.scl(2);
        assert_eq!(m, Matrix::from([[2, 4],[6, 8]]));
    }
}