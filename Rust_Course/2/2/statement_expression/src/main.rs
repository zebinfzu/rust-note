#[allow(unused)]
fn main() {
    // 1. rust是基于表达式的语言
    {
        fn add_with_extra(x: i32, y: i32) -> i32 {
            let x = x + 1; // 语句
            let y = y + 5; // 语句
            x + y // 表达式
        }
    }
    // 2. statement和expression
    {
        // let b = (let a = 8); // 语句不能作为赋值使用, 语句没有返回值
        let y = {
            let x = 3;
            x + 1 // 表达式有返回值, 表达式不能包含分号
        };
        // 如果表达式不返回值，就会隐式的返回一个单元
        assert_eq!(0, 0)
    }
    // 3. rust当中的{}代码块以及if else均为表达式，有返回值
    {
        let x = 1;
        let y = if x % 2 == 1 {
            "odd"
        } else {
            "even"
        };
    }
}
