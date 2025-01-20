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
}