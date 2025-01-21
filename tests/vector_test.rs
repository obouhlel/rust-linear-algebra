use rust_linear_algebra::vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut v1 = Vector::from([1, 3]);
        let v2 = Vector::from([2, 1]);

        v1.add(v2);

        assert_eq!(v1, Vector::from([3, 4]));
    }

    #[test]
    fn add_symbole() {
        let v1 = Vector::from([1, 3]);
        let v2 = Vector::from([2, 1]);

        assert_eq!(v1 + v2, Vector::from([3, 4]));
    }

    #[test]
    #[should_panic]
    fn add_panic() {
        let mut v1 = Vector::from([1, 3, 4]);
        let v2 = Vector::from([2, 1]);

        v1.add(v2);
    }

    #[test]
    fn sub() {
        let mut v1 = Vector::from([5, 7]);
        let v2 = Vector::from([2, 3]);

        v1.sub(v2);

        assert_eq!(v1, Vector::from([3, 4]));
    }

    #[test]
    fn sub_symbole() {
        let v1 = Vector::from([5, 7]);
        let v2 = Vector::from([2, 3]);

        assert_eq!(v1 - v2, Vector::from([3, 4]));
    }

    #[test]
    #[should_panic]
    fn sub_panic() {
        let mut v1 = Vector::from([1, 3, 4]);
        let v2 = Vector::from([2, 1]);

        v1.sub(v2);
    }

    #[test]
    fn scl() {
        let mut v = Vector::from([1, 2, 3]);
        v.scl(2);

        assert_eq!(v, Vector::from([2, 4, 6]));
    }

    #[test]
    fn scl_symbole() {
        let v = Vector::from([1, 2, 3]);

        assert_eq!(v * 2, Vector::from([2, 4, 6]));
    }

    #[test]
    fn dot_product_zero() {
        let u = Vector::from([0.0, 0.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u.dot(v), 0.0);
    }

    #[test]
    fn dot_product_two() {
        let u = Vector::from([1.0, 1.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u.dot(v), 2.0);
    }

    #[test]
    fn dot_product_nine() {
        let u = Vector::from([-1.0, 6.0]);
        let v = Vector::from([3.0, 2.0]);

        assert_eq!(u.dot(v), 9.0);
    }

    #[test]
    fn dot_symbole_product_zero() {
        let u = Vector::from([0.0, 0.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u * v, 0.0);
    }

    #[test]
    fn dot_symbole_product_two() {
        let u = Vector::from([1.0, 1.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u * v, 2.0);
    }

    #[test]
    fn dot_symbole_product_nine() {
        let u = Vector::from([-1.0, 6.0]);
        let v = Vector::from([3.0, 2.0]);

        assert_eq!(u * v, 9.0);
    }
}
