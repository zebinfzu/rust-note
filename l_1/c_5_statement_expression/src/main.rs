#![allow(unused)]
// 1. rust的函数返回值写在最后一行且不加分号，加分号会导致返回单元()
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn main() {
    // 2. rust 语句必须;结尾，语句没有返回值
    let a = 3;
    // 3. 表达式有返回值，如果加;就会变成语句，没有返回值了
    a + 1
    // 4. 如果表达式没有任何返回值，会隐式返回单元()
    // 5. if 语句块也是一个表达式，因此可以用于赋值
    let x = 3;
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者可以写作一行
    if z = if x % 2 == 1 { "odd" } else { "even" }
}

