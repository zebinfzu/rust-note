#![allow(unused)]
fn main() {
    // 变量命名rust推荐采用下划线和小写字母分割
    {
        let snake_style = 1;
    }
    // 变量绑定 -> 所有权规则
    // 变量可变性 默认不可变
    {
        let x = 5;
        // x = 6;
        // 加上mut关键字才可变
        let mut y = 5;
        y = 6;
    }

    // 下划线开头的变量不会报未使用的警告
    // 重复声明同名的变量，会shadow之前的变量
    {
        let _x = 5;
        let x = 3;
        let x = 6;
    }
    // 解构语法
    {
        let (a, mut b) = (true, false);
        // 1.59之后可以在解构语法的左侧使用元组、切片、结构体
        struct Struct {
            e: i32,
        }
        let (a, b, c, d, e);
        (a, b) = (1, 2);
        // _是一个占位符，表示后面还有一个值，但是我们不需要
        [c, .., d, _] = [1, 2, 3, 4, 5];
        Struct { e, .. } = Struct { e: 5 };
        println!("{a} {b} {c} {d} {e}");
    }
    // 常量用const定义，编译期就会被替换成硬编码的值，字母全大写
    // 类型必须标注
    {
        const MAX_POINTS: u32 = 100_000;
    }
    
}
