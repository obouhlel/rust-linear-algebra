use rust_linear_algebra::vector::Vector;
use rust_linear_algebra::vector::angle_cos::angle_cos;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_cos_1() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert!((angle_cos(&u, &v) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_angle_cos_2() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert!((angle_cos(&u, &v) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_angle_cos_3() {
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert!((angle_cos(&u, &v) - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_angle_cos_4() {
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert!((angle_cos(&u, &v) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_angle_cos_5() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert!((angle_cos(&u, &v) - 0.974631846).abs() < 1e-6);
    }

    #[test]
    #[should_panic(expected = "The vector need to be on the same plan")]
    fn test_angle_cos_panic_different_lengths() {
        let u = Vector::from([1., 2.]);
        let v = Vector::from([1., 2., 3.]);
        angle_cos(&u, &v);
    }

    #[test]
    #[should_panic(expected = "The vector can't be 0")]
    fn test_angle_cos_panic_zero_vector_u() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 2.]);
        angle_cos(&u, &v);
    }

    #[test]
    #[should_panic(expected = "The vector can't be 0")]
    fn test_angle_cos_panic_zero_vector_v() {
        let u = Vector::from([1., 2.]);
        let v = Vector::from([0., 0.]);
        angle_cos(&u, &v);
    }
}
