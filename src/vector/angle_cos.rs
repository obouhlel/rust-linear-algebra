use crate::vector::Vector;
use std::ops::{Add, Mul};

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Copy + Default + Mul<f32, Output = K> + Into<f32> + Add<Output = K> + Mul<Output = K>,
{
    if u.elements.len() != v.elements.len() {
        panic!("The vector need to be on the same plan");
    }

    let u_norm = u.norm();
    let v_norm = v.norm();

    if u_norm == 0.0 || v_norm == 0.0 {
        panic!("The vector can't be 0");
    }

    let dot_product: f32 = u.dot(v.clone()).into();
    dot_product / (u_norm * v_norm)
}
