use ndarray::arr2;

fn main() {
    // a 2d array with shape (2, 3)
    let a = arr2(&[[1., 2., 3.], [4., 5., 6.]]);
    let b = arr2(&[[1., 2., 3.], [4., 5., 6.]]);
    // sum the arrays
    let c = &a + &b;
    println!("{:?}", c);
}
