use rust_linear_algebra::linear_combinaison::linear_combination;
use rust_linear_algebra::vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let u = vec![
            Vector {
                elements: vec![1, 2, 3],
            },
            Vector {
                elements: vec![4, 5, 6],
            },
            Vector {
                elements: vec![7, 8, 9],
            },
        ];
        let coefs = vec![1, 2, 3];
        let result = linear_combination(&u, &coefs);
        assert_eq!(result.elements, vec![30, 36, 42]);
    }

    #[test]
    #[should_panic]
    fn test_case_2() {
        let u = vec![
            Vector {
                elements: vec![1, 2, 3],
            },
            Vector {
                elements: vec![4, 5, 6],
            },
        ];
        let coefs = vec![1, 2, 3];
        linear_combination(&u, &coefs);
    }
}
