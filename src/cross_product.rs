use crate::vector::Vector;
use std::ops::{Mul, Sub};

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Copy + Default + Sub<Output = K> + Mul<Output = K>,
{
    if u.elements.len() != 3 && v.elements.len() != 3 {
        panic!("Both vectors must have exactly 3 elements for the cross product.");
    }

    let w_x = u.elements[1] * v.elements[2] - u.elements[2] * v.elements[1];
    let w_y = u.elements[2] * v.elements[0] - u.elements[0] * v.elements[2];
    let w_z = u.elements[0] * v.elements[1] - u.elements[1] * v.elements[0];

    Vector {
        elements: vec![w_x, w_y, w_z],
    }
}
