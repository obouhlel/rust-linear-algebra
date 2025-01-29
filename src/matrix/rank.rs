use super::Matrix;
use crate::minus_one::MinusOne;
use crate::one::One;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub, SubAssign};

impl<K> Matrix<K>
where
    K: Debug
        + Copy
        + One
        + MinusOne
        + Default
        + PartialEq
        + Add<Output = K>
        + Sub<Output = K>
        + SubAssign<K>
        + Mul<Output = K>
        + Div<Output = K>,
{
    pub fn rank(&self) -> usize {
        let result = self.row_echelon();
        let mut rank = 0;

        for row in 0..result.rows() {
            if result.elements[row].iter().any(|&x| x != K::default()) {
                rank += 1;
            }
        }

        rank
    }
}
