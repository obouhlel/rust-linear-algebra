#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector() {
        let v1 = Vector { elements: vec![1, 2] };
        let v2 = Vector { elements: vec![3, 4] };

        let v3 = v1 + v2;
    }
}