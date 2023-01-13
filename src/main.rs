// use approx crate to solve the problem of floating point comparison
use approx::assert_abs_diff_eq;
use ndarray::{arr1, arr2, Array, Array1};

fn main() {
    // a 2d array with shape (2, 3)
    let a = arr2(&[[1., 2., 3.], [4., 5., 6.]]);
    let b = arr2(&[[1., 2., 3.], [4., 5., 6.]]);
    // sum the arrays
    let sum = &a + &b;
    println!("{:?}", sum);

    // multiple a and b array using the dot method
    {
        let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

        let b = arr2(&[[6, 3], [5, 2], [4, 1]]);

        let mul = a.dot(&b);
        println!("{:?}", mul);
    }

    /*****************************
     * Multiply scalar with Vector
     *****************************/

    let scalar = 4;

    let vector = arr1(&[1, 2, 3]);

    let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);

    /*******************
     * Vector comparison
     *******************/

    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;
    // TODO: fix error
    // assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));
}
