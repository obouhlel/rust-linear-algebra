use crate::vector::Vector;
// use crate::one::One;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::slice::{Iter, IterMut};
use std::usize;

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub elements: Vec<Vec<K>>,
}

impl<K> From<Vec<Vec<K>>> for Matrix<K> {
    fn from(elements: Vec<Vec<K>>) -> Self {
        Matrix { elements }
    }
}

impl<K, const ROWS: usize, const COLS: usize> From<[[K; COLS]; ROWS]> for Matrix<K>
where
    K: Clone,
{
    fn from(array: [[K; COLS]; ROWS]) -> Self {
        let elements = array.iter().map(|row| row.to_vec()).collect();
        Matrix { elements }
    }
}

impl<K> Matrix<K> {
    pub fn iter(&self) -> Iter<Vec<K>> {
        self.elements.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<Vec<K>> {
        self.elements.iter_mut()
    }
}

impl<K> Matrix<K> {
    pub fn new(elements: Vec<Vec<K>>) -> Self {
        Matrix { elements }
    }
}

impl<K> Matrix<K> {
    pub fn rows(&self) -> usize {
        self.elements.len()
    }
    pub fn cols(&self) -> usize {
        self.elements[0].len()
    }
}

impl<K> PartialEq for Matrix<K>
where
    Vec<K>: PartialEq,
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<K> Add for Matrix<K>
where
    K: Add<Output = K> + Copy + Default,
{
    type Output = Matrix<K>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            panic!("Matrices must have the same dimensions");
        }

        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i][j] = self.elements[i][j] + rhs.elements[i][j];
            }
        }

        Matrix { elements }
    }
}

impl<K> Sub for Matrix<K>
where
    K: Sub<Output = K> + Copy + Default,
{
    type Output = Matrix<K>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            panic!("Matrices must have the same dimensions");
        }

        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i][j] = self.elements[i][j] - rhs.elements[i][j];
            }
        }

        Matrix { elements }
    }
}

impl<K, T> Mul<T> for Matrix<K>
where
    K: Mul<T, Output = K> + Copy + Default,
    T: Copy,
{
    type Output = Matrix<K>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut elements = vec![vec![K::default(); self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i][j] = self.elements[i][j] * rhs;
            }
        }

        Matrix { elements }
    }
}

impl<K> Mul<Vector<K>> for Matrix<K>
where
    K: Copy + Default + Add<Output = K> + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, rhs: Vector<K>) -> Self::Output {
        if self.elements.is_empty() || rhs.elements.is_empty() {
            panic!("Matrix or vector is empty. Cannot perform multiplication.");
        }
        if self.cols() != rhs.len() {
            panic!(
                "Incompatible dimensions for matrix-vector multiplication. \
                Matrix has {} columns, but vector has {} elements.",
                self.cols(),
                rhs.len()
            );
        }

        let mut elements = vec![K::default(); self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[i] = elements[i] + self.elements[i][j] * rhs.elements[j];
            }
        }

        Vector { elements }
    }
}

impl<K> Mul for Matrix<K>
where
    K: Mul<Output = K> + Add<Output = K> + Copy + Default,
{
    type Output = Matrix<K>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.elements.is_empty() || rhs.elements.is_empty() {
            panic!("One of the matrices is empty. Cannot perform multiplication.");
        }
        if self.cols() != rhs.rows() {
            panic!(
                "Incompatible dimensions for matrix multiplication. \
                Left matrix has {} columns, but right matrix has {} rows.",
                self.cols(),
                rhs.rows()
            );
        }

        let mut elements = vec![vec![K::default(); rhs.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                for k in 0..self.cols() {
                    elements[i][j] = elements[i][j] + (self.elements[i][k] * rhs.elements[k][j]);
                }
            }
        }

        Matrix { elements }
    }
}

impl<K> Matrix<K>
where
    K: Add<Output = K> + Copy,
{
    pub fn add(&mut self, m: Matrix<K>) {
        if self.rows() != m.rows() || self.cols() != m.cols() {
            panic!("Matrices must have the same dimensions");
        }

        self.iter_mut()
            .zip(m.iter())
            .for_each(|(row_self, row_other)| {
                row_self
                    .iter_mut()
                    .zip(row_other.iter())
                    .for_each(|(a, b)| *a = *a + *b)
            });
    }
}

impl<K> Matrix<K>
where
    K: Sub<Output = K> + Copy,
{
    pub fn sub(&mut self, m: Matrix<K>) {
        if self.rows() != m.rows() || self.cols() != m.cols() {
            panic!("Matrices must have the same dimensions");
        }

        self.iter_mut()
            .zip(m.iter())
            .for_each(|(row_self, row_other)| {
                row_self
                    .iter_mut()
                    .zip(row_other.iter())
                    .for_each(|(a, b)| *a = *a - *b)
            });
    }
}

impl<K> Matrix<K>
where
    K: Mul<Output = K> + Copy,
{
    pub fn scl(&mut self, a: K) {
        self.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v = *v * a));
    }
}

impl<K> Matrix<K>
where
    K: Mul<Output = K> + Add<Output = K> + Copy + Default,
{
    pub fn mul_mat(&self, mat: Matrix<K>) -> Matrix<K> {
        self.clone() * mat
    }
    pub fn mul_vec(&self, vec: Vector<K>) -> Vector<K> {
        self.clone() * vec
    }
}

impl<K> Matrix<K>
where
    K: Copy + Default + Add<Output = K>,
{
    pub fn trace(&self) -> K {
        if self.cols() != self.rows() {
            panic!("Matrix must be a square");
        }

        (1..self.rows()).fold(self.elements[0][0], |acc, i| acc + self.elements[i][i])
    }
}

impl<K> Matrix<K>
where
    K: Copy + Default,
{
    pub fn transpose(&self) -> Matrix<K> {
        let mut elements = vec![vec![K::default(); self.rows()]; self.cols()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                elements[j][i] = self.elements[i][j];
            }
        }

        Matrix { elements }
    }
}

// Algorithme: Gauss-Jordan
impl<K> Matrix<K>
where
    K: Debug
        + Copy
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

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.elements.swap(row1, row2);
    }

    fn div_row(&mut self, row: usize, scalar: K) {
        self.elements[row]
            .iter_mut()
            .for_each(|element| *element = *element / scalar);
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        if self.elements.is_empty() || self.elements[0].is_empty() {
            return self.clone();
        }

        let mut result = self.clone();
        let mut pivot_row = 0;

        for col in 0..result.cols() {
            if let Some(pivot) = result.find_pivot(pivot_row, col) {
                if pivot != pivot_row {
                    result.swap_rows(pivot_row, pivot);
                }

                let pivot_value = result.elements[pivot_row][col];
                if pivot_value == K::default() {
                    continue;
                }

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

// impl<K> Matrix<K>
// where
//     K: Copy + Default + One + Add<Output = K> + Sub<Output = K> + Mul<Output = K>,
// {
//     // ad−bc
//     fn det_matrix_2x2(&self) -> K {
//         self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0]
//     }

//     // a(ei−fh)−b(di−fg)+c(dh−eg)
//     fn det_matrix_3x3(&self) -> K {
//         self.elements[0][0]
//             * (self.elements[1][1] * self.elements[2][2]
//                 - self.elements[1][2] * self.elements[2][1])
//             - self.elements[0][1]
//                 * (self.elements[1][0] * self.elements[2][2]
//                     - self.elements[1][2] * self.elements[2][0])
//             + self.elements[0][2]
//                 * (self.elements[1][0] * self.elements[2][1]
//                     - self.elements[1][1] * self.elements[2][0])
//     }

//     // fn det_matrix_4x4(&self) -> K {
//     //     let mut det = K::one();


//     // }

//     fn det_matrix_nxn(&self) -> K {
//         let mut det = K::one();
//     }

//     pub fn determinant(&self) -> K {
//         if self.cols() != self.rows() {
//             panic!("Matrix must be a square");
//         }

//         let dim = self.cols();

//         match dim {
//             0 => panic!("Empty matrix"),
//             1 => self.elements[0][0],
//             2 => self.det_matrix_2x2(),
//             3 => self.det_matrix_3x3(),
//             // 4 => self.det_matrix_4x4(),
//             _ => self.det_matrix_nxn(),
//         }
//     }
// }
