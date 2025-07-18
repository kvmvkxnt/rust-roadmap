fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

// fn main() {
//     // addition
//     let sum = 5 + 10;
//
//     // substraction
//     let difference = 95.5 - 4.3;
//
//     // multiplication
//     let product = 4 * 30;
//
//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3;
//
//     // remainder
//     let remainder = 43 % 5;
// }

// fn main() {
//     let t = true;
//     let f: bool = false; // with explicit type annotation
// }

// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// fn main() {
//     let tup = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("The value of y is: {y}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
//     let first = a[0];
//     let second = a[1];
// }

// use std::io;
//
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
//     println!("Please enter an array index.");
//
//     let mut index = String::new();
//
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//
//     let element = a[index];
//
//     println!("The value of the element at index {index} is: {element}");
// }
