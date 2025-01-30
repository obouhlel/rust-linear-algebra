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
    pub fn row_echelon(&self) -> Matrix<K> {
        if self.elements.is_empty() || self.elements[0].is_empty() {
            return self.clone();
        }
        if self.is_row_echelon_form() {
            return self.clone();
        }
        self.gaussian_elimination(None, None)
    }
}

impl<K> Matrix<K>
where
    K: Debug + Copy + Default + PartialEq,
{
    pub fn is_row_echelon_form(&self) -> bool {
        let mut last_pivot_col = None;

        for row in 0..self.rows() {
            let pivot_col = self.elements[row].iter().position(|&x| x != K::default());

            match pivot_col {
                Some(col) => {
                    if let Some(last_col) = last_pivot_col {
                        if col <= last_col {
                            return false;
                        }
                    }
                    last_pivot_col = Some(col);

                    for below_row in row + 1..self.rows() {
                        if self.elements[below_row][col] != K::default() {
                            return false;
                        }
                    }
                }
                None => {
                    for below_row in row + 1..self.rows() {
                        if self.elements[below_row].iter().any(|&x| x != K::default()) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}
