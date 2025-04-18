use super::identity::identity_mat;
use super::Matrix;
use crate::num::MinusOne;
use crate::num::One;
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
    pub fn inverse(&self) -> Result<Matrix<K>, &str> {
        if self.cols() != self.rows() {
            return Err("Matrix must be a square");
        }
        let det = self.determinant();
        if det == K::default() {
            return Err("Matrix is singular and cannot be inverted");
        }
        let mut identity = identity_mat::<K>(self.cols(), self.rows());
        let _ = self.gaussian_elimination(None, Some(&mut identity));
        Ok(identity)
    }
}
