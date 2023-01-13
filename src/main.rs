use ndarray::{arr1, arr2, Array1};

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
}
