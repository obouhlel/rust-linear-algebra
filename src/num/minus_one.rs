pub trait MinusOne {
    fn minus_one() -> Self;
}

impl MinusOne for i8 {
    fn minus_one() -> Self {
        -1
    }
}

impl MinusOne for i16 {
    fn minus_one() -> Self {
        -1
    }
}

impl MinusOne for i32 {
    fn minus_one() -> Self {
        -1
    }
}

impl MinusOne for i64 {
    fn minus_one() -> Self {
        -1
    }
}

impl MinusOne for i128 {
    fn minus_one() -> Self {
        -1
    }
}

impl MinusOne for isize {
    fn minus_one() -> Self {
        -1
    }
}

impl MinusOne for f32 {
    fn minus_one() -> Self {
        -1.0
    }
}

impl MinusOne for f64 {
    fn minus_one() -> Self {
        -1.0
    }
}
