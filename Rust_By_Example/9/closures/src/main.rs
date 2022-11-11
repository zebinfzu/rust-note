#[allow(unused)]
#[allow(dead_code)]
fn main() {
    // function
    fn function(i: i32) -> i32 {
        i + 1
    }
    // closure
    let closure = |i: i32| -> i32 {i + 1};
    // 入参类型和返回值类型都可以不写，让编译器推导
    let closure = |i| i + 1;
    println!("{}", closure(1));
}
