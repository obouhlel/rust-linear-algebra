use rust_linear_algebra::vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_1() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.0);
    
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.0);
    
        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.0);
    }
    
    #[test]
    fn test_norm() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm(), 0.0);
    
        let u = Vector::from([1., 2., 3.]);
        assert!((u.norm() - 3.74165738).abs() < 1e-6);
    
        let u = Vector::from([-1., -2.]);
        assert!((u.norm() - 2.236067977).abs() < 1e-6);
    }
    
    #[test]
    fn test_norm_inf() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_inf(), 0.0);
    
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_inf(), 3.0);
    
        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_inf(), 2.0);
    }
}
