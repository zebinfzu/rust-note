#![allow(unused)]
fn main() {
    // statically typed
    let guess: u32 = "42".parse().expect("Not a number!");
    // scalar type: int float boolean char
    // int: i8~i128 u8~u128 default i32
    // float: f32 f64 default f64

    // compound type: tuple array
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching -> destructure
    let (x, y, z) = tup;

    // use .
    let five_hundred = tup.0;

    // unit ()

    // array [type: length]
    let a = [1, 2, 3, 4, 5];
    // 指定初始值和长度
    let a = [3; 5];

    use std::io;

    let a = [0, 1, 2, 3, 4, 5];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
