use std::io;

fn foo(num1: u32, num2: u32) -> u32 {
    num1 * num1 + num2 * num2
}
fn main() {
    let (mut num1, mut num2) = (String::new(), String::new());
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num1: u32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("panic num1")
    };
    let num2: u32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("panic num1")
    };
    let num3 = foo(num1, num2);
    println!("{0} * {0} + {1} * {1} = {2}", num1, num2, num3);
}
