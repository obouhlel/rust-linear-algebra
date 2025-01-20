use crate::vector::Vector;
use std::{ops::{Add, Mul}, vec};

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Copy + Default + Add<Output = K> + Mul<Output = K>
{
    if u.len() != coefs.len() {
        panic!("Need the same size");
    }

    let mut result = Vector {
        elements: vec![K::default(); u[0].elements.len()]
    };

    for (vector, &coef) in u.iter().zip(coefs) {
        for (res_elem, &vec_elem) in result.iter_mut().zip(&vector.elements) {
            *res_elem = *res_elem + vec_elem * coef;
        }
    }

    result
}