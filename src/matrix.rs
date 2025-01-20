// use std::ops::Add;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub elements: Vec<Vec<K>>,
}

impl<K> From<Vec<Vec<K>>> for Matrix<K> {
    fn from(elements: Vec<Vec<K>>) -> Self {
        Matrix { elements }
    }
}

// impl<K> PartialEq for Matrix<K> {
//     fn eq(&self, other: &Self) -> bool {
//         self.elements == other.elements
//     }
//     fn ne(&self, other: &Self) -> bool {
//         self.elements == other.elements
//     }
// }
