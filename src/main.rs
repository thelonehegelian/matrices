// use approx crate to solve the problem of floating point comparison
use approx::assert_abs_diff_eq;
use nalgebra::Matrix3;
use ndarray::{arr1, arr2, Array, Array1};
extern crate nalgebra;
use nalgebra::DMatrix;
use serde_json;

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

    /******************
     * INVERT A MATRIX
     ******************/
    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    println!("m1 = {}", m1);
    match m1.try_inverse() {
        Some(v) => println!("m1^-1 = {}", v),
        None => println!("m1 is not invertible"),
    };

    /**********************************************
     * SERIALIZATION AND DESERIALIZATION OF MATRIX
     **********************************************/

    let row_slice: Vec<i32> = (1..5001).collect();
    // Dmatrix from row slice
    let matrix = DMatrix::from_row_slice(100, 50, &row_slice);
    // TODO : Fix errors
    // serialize matrix
    // let serialized_matrix = serde_json::to_string(&matrix).unwrap();
    // // deserialize matrix
    // let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix).unwrap();
}
