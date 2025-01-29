use rust_linear_algebra::matrix::Matrix;

fn main() {
    let u = Matrix::from([[1., 0., 1.], [0., 2., 1.], [1., 1., 1.]]);

    println!("u = {:?}", u.inverse());
}
