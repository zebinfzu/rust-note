// 全局变量声明在所有其他作用域之外
// static 具有'static 生命周期，可以是可变的变量，需要使用static mut 声明
static LANGUAGE: &'static str = "Rust";
// const 不可变的值
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}



fn main() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}
