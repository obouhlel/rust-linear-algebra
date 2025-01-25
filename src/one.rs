// Définition du trait
pub trait One {
    fn one() -> Self;
}

// Implémentations pour les entiers signés
impl One for i8 {
    fn one() -> Self {
        1
    }
}
impl One for i16 {
    fn one() -> Self {
        1
    }
}
impl One for i32 {
    fn one() -> Self {
        1
    }
}
impl One for i64 {
    fn one() -> Self {
        1
    }
}
impl One for i128 {
    fn one() -> Self {
        1
    }
}
impl One for isize {
    fn one() -> Self {
        1
    }
}

// Implémentations pour les entiers non-signés
impl One for u8 {
    fn one() -> Self {
        1
    }
}
impl One for u16 {
    fn one() -> Self {
        1
    }
}
impl One for u32 {
    fn one() -> Self {
        1
    }
}
impl One for u64 {
    fn one() -> Self {
        1
    }
}
impl One for u128 {
    fn one() -> Self {
        1
    }
}
impl One for usize {
    fn one() -> Self {
        1
    }
}

// Implémentations pour les flottants
impl One for f32 {
    fn one() -> Self {
        1.0
    }
}
impl One for f64 {
    fn one() -> Self {
        1.0
    }
}
