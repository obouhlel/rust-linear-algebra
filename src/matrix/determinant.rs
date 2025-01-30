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
    // ad−bc
    fn det_matrix_2x2(&self) -> K {
        self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0]
    }

    // a(ei−fh)−b(di−fg)+c(dh−eg)
    fn det_matrix_3x3(&self) -> K {
        self.elements[0][0]
            * (self.elements[1][1] * self.elements[2][2]
                - self.elements[1][2] * self.elements[2][1])
            - self.elements[0][1]
                * (self.elements[1][0] * self.elements[2][2]
                    - self.elements[1][2] * self.elements[2][0])
            + self.elements[0][2]
                * (self.elements[1][0] * self.elements[2][1]
                    - self.elements[1][1] * self.elements[2][0])
    }

    fn det_matrix_4x4(&self) -> K {
        let a = &self.elements;

        let det_m11 = a[1][1] * (a[2][2] * a[3][3] - a[2][3] * a[3][2])
            - a[1][2] * (a[2][1] * a[3][3] - a[2][3] * a[3][1])
            + a[1][3] * (a[2][1] * a[3][2] - a[2][2] * a[3][1]);

        let det_m12 = a[1][0] * (a[2][2] * a[3][3] - a[2][3] * a[3][2])
            - a[1][2] * (a[2][0] * a[3][3] - a[2][3] * a[3][0])
            + a[1][3] * (a[2][0] * a[3][2] - a[2][2] * a[3][0]);

        let det_m13 = a[1][0] * (a[2][1] * a[3][3] - a[2][3] * a[3][1])
            - a[1][1] * (a[2][0] * a[3][3] - a[2][3] * a[3][0])
            + a[1][3] * (a[2][0] * a[3][1] - a[2][1] * a[3][0]);

        let det_m14 = a[1][0] * (a[2][1] * a[3][2] - a[2][2] * a[3][1])
            - a[1][1] * (a[2][0] * a[3][2] - a[2][2] * a[3][0])
            + a[1][2] * (a[2][0] * a[3][1] - a[2][1] * a[3][0]);

        a[0][0] * det_m11 - a[0][1] * det_m12 + a[0][2] * det_m13 - a[0][3] * det_m14
    }

    fn det_matrix_nxn(&self) -> K {
        if !self.is_row_echelon_form() {
            let mut det = K::one();
            let _ = self.gaussian_elimination(Some(&mut det), None);
            return det;
        }
        let mut det = K::one();
        for i in 0..self.rows() {
            det = det * self.elements[i][i];
        }
        det
    }

    pub fn determinant(&self) -> K {
        if self.cols() != self.rows() {
            panic!("Matrix must be square");
        }

        let dim = self.cols();

        match dim {
            0 => panic!("Empty matrix"),
            1 => self.elements[0][0],
            2 => self.det_matrix_2x2(),
            3 => self.det_matrix_3x3(),
            4 => self.det_matrix_4x4(),
            _ => self.det_matrix_nxn(),
        }
    }
}
