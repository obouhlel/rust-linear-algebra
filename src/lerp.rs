use std::ops::{Add, Mul};

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Add<Output = V> + Mul<f32, Output = V>,
{
    if t < 0.0 || t > 1.0 {
        panic!("t need to be beteen 0.0 to 1.0");
    }

    u * (1.0 - t) + v * t
}
