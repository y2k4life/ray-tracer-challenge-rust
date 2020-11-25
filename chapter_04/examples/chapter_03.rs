use rustic_ray::{Matrix, Point, IDENTITY};

#[rustfmt::skip]
fn main() {
    let mut i = IDENTITY;

    println!("What happens when you invert the identity matrix?");
    println!();
    println!("Identity Matrix:");
    println!("{}", i);

    println!("Inverted Identity Matrix:");
    println!("{}", i.inverse());

    println!("--------------------------------------------------------------");
    println!("What do you get when you multiply a matrix by its inverse?");
    println!();
    let mut a = Matrix::new([
        [ 3.0, -9.0,  7.0,  3.0],
        [ 3.0, -8.0,  2.0, -9.0],
        [-4.0,  4.0,  4.0,  1.0],
        [-6.0,  5.0, -1.0,  1.0],
    ]);

    println!("Matrix A:");
    println!("{}", a);
    println!("Inverse of matrix A:");
    println!("{}", a.inverse());
    println!("Matrix A multiplied by its inverse:");
    println!("{}", a * a.inverse());

    println!("--------------------------------------------------------------");
    println!("Is there any difference between:");
    println!();
    println!("  inverse of the transpose of matrix A");
    println!("            vs");
    println!("  transpose of the inverse of matrix A ");
    println!();
    println!("Inverse of the transpose of a matrix A");
    println!("{}", a.transpose().inverse());

    println!("Transpose of the inverse of a matrix A");
    println!("{}", a.inverse().transpose());

    println!("--------------------------------------------------------------");
    let mut i2 = Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
        
    let p = Point::new(1.0, 2.0, 3.0);

    println!("Multiplying the identity matrix by a point (tuple):");
    println!();
    println!("Point:");
    println!("{}", p);
    println!();
    println!("Multiplied by identity matrix:");
    println!();
    println!("{}", i * p);
    println!();
    println!("Changed identity matrix:");
    println!();
    println!("{}", i2);
    println!();
    println!("Multiplied by change identity matrix:");
    println!();
    println!("{}", i2 * p);
    
    i2[0][0] = 3.0;
    i2[3][3] = 3.0;
    println!();
    println!("Changed identity matrix:");
    println!();
    println!("{}", i2);
    println!();
    println!("Multiplied by change identity matrix:");
    println!();
    println!("{}", i2 * p);
}
