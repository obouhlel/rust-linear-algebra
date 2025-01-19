use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub elements: Vec<Vec<K>>,
}