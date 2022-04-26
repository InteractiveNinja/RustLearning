use std::fmt;
use std::fmt::{Formatter, write};

struct Matrix(f32, f32, f32, f32);
/// Kann ein Matrix Struct umdrehen
fn reverse(c: Matrix) -> Matrix {
    // deconstruct, kinda sad bei einem struct lol
    let Matrix { 0: c1, 1: c2, 2: c3, 3: c4 } = c;
    Matrix(c4,c3,c2,c1)
}


impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix Struct: ({} {} {} {})", self.0, self.1, self.2, self.3)
    }
}

pub fn run() {
    let m = Matrix(9.4,2.1,2.4,3.2);

    println!("Normal {}",m);
    println!("Reverse {}",reverse(m));
}