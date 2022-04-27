use std::fmt;
use std::fmt::{Formatter, write};

struct Matrix(f32, f32, f32, f32);

struct Point {
    x: f32,
    y: f32,
}


struct Rectangle {
    height: i32,
    width: i32
}
/// Kann ein Matrix Struct umdrehen
fn reverse(c: Matrix) -> Matrix {
    // deconstruct, kinda sad bei einem struct lol
    let Matrix { 0: c1, 1: c2, 2: c3, 3: c4 } = c;
    Matrix(c4,c3,c2,c1)
}

fn rect_area(rec: Rectangle) -> i32 {
    let Rectangle {height,width } = rec;
    height * width
}

fn square(point: Point,size: i32) -> Rectangle {
    Rectangle{height: point.x as i32 * size ,width: point.y as i32 * size}
}


impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix Struct: ({} {} {} {})", self.0, self.1, self.2, self.3)
    }
}


pub fn run() {
    let m = Matrix(9.4,2.1,2.4,3.2);
    let r = Rectangle{height: 10, width: 5};

    println!("Normal {}",m);
    println!("Reverse {}",reverse(m));

    println!("Rect Height {} and Width {}",r.height,r.width);
    println!("Rect Area {}",rect_area(r))

}