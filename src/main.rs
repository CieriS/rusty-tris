pub mod matrix;
use matrix::Matrix as m;
use crate::matrix::Matrix;

fn main() {
    let mut turn:bool = true;
    let mut mat = m::default();
    mat.set(0, 0, turn);


    println!("{:#?}", mat);
}
