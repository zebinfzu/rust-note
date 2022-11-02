#![allow(unused)]
fn main() {
    // statement 没有返回值
    {
        // let 是语句所以必须;结尾
        let a = 8;
        let b: Vec<f64> = Vec::new();
        let (a, c) = ("hi", false);
    }
    // expression
    {
        // 代码块是表达式，有返回值
        let y = {
            let x = 3;
            x
        };
    }
    // 如果表达式没有显示的返回值，则会隐式的返回一个单元
    
}
