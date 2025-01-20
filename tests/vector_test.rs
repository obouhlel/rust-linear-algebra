use rust_linear_algebra::vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut v1 = Vector { elements: vec![1, 3] };
        let v2 = Vector { elements: vec![2, 1] };

        v1.add(v2);

        assert_eq!(v1, Vector { elements: vec![3, 4] });
    }

    #[test]
    #[should_panic]
    fn add_not_same_plan() {
        let mut v1 = Vector { elements: vec![1, 3, 4] };
        let v2 = Vector { elements: vec![2, 1] };

        v1.add(v2);
    }

    #[test]
    fn sub() {
        let mut v1 = Vector { elements: vec![5, 7] };
        let v2 = Vector { elements: vec![2, 3] };

        v1.sub(v2);

        assert_eq!(v1, Vector { elements: vec![3, 4] });
    }

    #[test]
    #[should_panic]
    fn sub_not_same_plan() {
        let mut v1 = Vector { elements: vec![1, 3, 4] };
        let v2 = Vector { elements: vec![2, 1] };

        v1.sub(v2);
    }

    #[test]
    fn scl() {
        let mut v = Vector { elements: vec![1, 2, 3] };
        v.scl(2);

        assert_eq!(v, Vector { elements: vec![2, 4, 6] });
    }
}