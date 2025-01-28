use super::Matrix;
use crate::minus_one::MinusOne;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

impl<K> Matrix<K>
where
    K: Debug
        + Copy
        + MinusOne
        + Default
        + PartialEq
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>,
{
    fn find_pivot(&self, start_row: usize, col: usize) -> Option<usize> {
        for row in start_row..self.rows() {
            if self.elements[row][col] != K::default() {
                return Some(row);
            }
        }
        None
    }

    fn div_row(&mut self, row: usize, scalar: K) {
        self.elements[row]
            .iter_mut()
            .for_each(|element| *element = *element / scalar);
    }

    pub(crate) fn gaussian_elemination(&self, mut nb_swap: Option<&mut usize>) -> Matrix<K> {
        if self.elements.is_empty() || self.elements[0].is_empty() {
            return self.clone();
        }

        let mut result = self.clone();
        let mut pivot_row = 0;

        for col in 0..result.cols() {
            if let Some(pivot) = result.find_pivot(pivot_row, col) {
                if pivot != pivot_row {
                    result.elements.swap(pivot_row, pivot);
                    if let Some(ref mut swap) = nb_swap {
                        **swap += 1;
                    }
                }

                let pivot_value = result.elements[pivot_row][col];
                result.div_row(pivot_row, pivot_value);

                for row in 0..result.rows() {
                    if row == pivot_row {
                        continue;
                    }

                    let factor = result.elements[row][col];
                    if factor == K::default() {
                        continue;
                    }

                    for column in col..result.cols() {
                        result.elements[row][column] = result.elements[row][column]
                            - (factor * result.elements[pivot_row][column]);
                    }
                }

                pivot_row += 1;
            }
        }

        result
    }
}
