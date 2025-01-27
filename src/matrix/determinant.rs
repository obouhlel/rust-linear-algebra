use super::Matrix;
use crate::one::One;
use std::ops::{Add, Mul, Sub};

impl<K> Matrix<K>
where
    K: Copy + Default + One + Add<Output = K> + Sub<Output = K> + Mul<Output = K>,
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

    // fn det_matrix_4x4(&self) -> K {
    //     let mut det = K::one();

    // }

    fn det_matrix_nxn(&self) -> K {
        let det = K::one();
        det
    }

    pub fn determinant(&self) -> K {
        if self.cols() != self.rows() {
            panic!("Matrix must be a square");
        }

        let dim = self.cols();

        match dim {
            0 => panic!("Empty matrix"),
            1 => self.elements[0][0],
            2 => self.det_matrix_2x2(),
            3 => self.det_matrix_3x3(),
            // 4 => self.det_matrix_4x4(),
            _ => self.det_matrix_nxn(),
        }
    }
}
