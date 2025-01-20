use std::ops::{Add, Mul};

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Add<Output = V> + Mul<f32, Output = V>,
{
    match t {
        0.0 => u,
        1.0 => v,
        t if t < 0.0 || t > 1.0 => panic!("t must be between 0 and 1"),
        _ => u * (1.0 - t) + v * t,
    }
}
