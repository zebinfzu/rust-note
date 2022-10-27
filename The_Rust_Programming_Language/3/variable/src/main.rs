#![allow(unused)]
fn main() {
    // immutable
    let x = 1;
    // shadow
    // mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // shadow
    let spaces = "    ";
    let spaces = spaces.len();
}
