use super::Vector;
use std::ops::Mul;

impl<K> Vector<K>
where
    K: Mul<f32, Output = K> + Into<f32> + Copy + Default,
{
    pub fn norm_1(&self) -> f32 {
        self.iter().fold(0.0, |acc, &x| {
            let x_f32: f32 = x.into();
            if x_f32 < 0.0 {
                acc + (-x_f32)
            } else {
                acc + x_f32
            }
        })
    }
    pub fn norm(&self) -> f32 {
        let sum_of_squares: f32 = self.elements.iter().fold(0.0, |acc, &x| {
            let x_f32: f32 = x.into();
            acc + x_f32.powi(2)
        });
        sum_of_squares.sqrt()
    }
    pub fn norm_inf(&self) -> f32 {
        self.elements.iter().fold(0.0, |acc, &x| {
            let x_f32: f32 = x.into();
            acc.max(x_f32.abs())
        })
    }
}