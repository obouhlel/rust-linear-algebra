use rust_linear_algebra::vector::Vector;
use rust_linear_algebra::cross_product::cross_product;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product_1() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        assert_eq!(cross_product(&u, &v), Vector::from([0., 1., 0.]));
    }

    #[test]
    fn test_cross_product_2() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(cross_product(&u, &v), Vector::from([-3., 6., -3.]));
    }

    #[test]
    fn test_cross_product_3() {
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        assert_eq!(cross_product(&u, &v), Vector::from([17., -58., -16.]));
    }

    #[test]
    #[should_panic]
    fn test_cross_product_panic_1() {
        let u = Vector::from([1., 2.]);
        let v = Vector::from([3., 4.]);
        cross_product(&u, &v);
    }

    #[test]
    #[should_panic]
    fn test_cross_product_panic_2() {
        let u = Vector::from([1., 2., 3., 4.]);
        let v = Vector::from([5., 6., 7., 8.]);
        cross_product(&u, &v);
    }
}