# Rust Linear Algebra Library

A comprehensive linear algebra library implemented in Rust, providing efficient and safe mathematical operations.

## Features

- Vector operations
- Matrix operations
- Linear transformations
- Basic algebraic operations
- Type-safe implementations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-linear-algebra = "0.1.1"
```

## Usage

```rust
use rust_linear_algebra::{Matrix, Vector};

// Create a vector
let v = Vector::from([1.0, 2.0, 3.0]);

// Create a matrix
let m = Matrix::from([
    [1.0, 0.0],
    [0.0, 1.0],
]);
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
