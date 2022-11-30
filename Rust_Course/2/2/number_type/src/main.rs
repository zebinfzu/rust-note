fn main() {
    // 1. rust数值类型比较多，可以按照实际需求实现最优的性能
    // 2. i8 1byte -> isize 默认情况下整数数字字面量推导为i32
    // 3. u8 1byte -> usize
    // 4. f32, f64
    // 5. number literal
    {
        let _n = 98_222; // 十进制字面量
        let _n = 0xff; // 十六进制字面量
        let _n = 0o77; // 八进制字面量
        let _n = 0b1111_0000; // 二进制字面量
        let _n = b'A'; // 字节 -> u8 对应ascii码
    }

    // 6. 整数溢出在debug模式下会panic
    // 7. 整数溢出在release模式下默认按照补码循环处理溢出
    // 8. 可以显示的处理数值溢出
    // wrapping_* 方法在所有模式下都按照补码循环溢出规则处理
    // checked_* 方法时发生溢出，则返回 None 值
    // overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    // saturating_* 方法使值达到最小值或最大值
    {
        let a: u8 = u8::MAX.wrapping_add(1);
        println!("{}", a);
        let b = i32::MAX.checked_add(1);
        if let None = b {
            println!("加法产生了溢出");
        }
    }

    // 9. 浮点数类型默认推导为f64
    // 10. 浮点数只是近似表示，rust同c，浮点数都是ieee754标准的实现
    // 11. 浮点数没有实现std::cmp::Eq，因此不可以使用==运算符
    {
        let _x = 3.0;
        let _x = 3.0_f32;
    }

    // 12. NaN Rust用这个来处理浮点数未定义的数字结果
    {
        let x = (-42.0_f32).sqrt();
        // assert_eq!(x, x); // 所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较，这里会panic
        // 要获得一个值是不是NaN可以使用is_nan方法
        if x.is_nan() {
            println!("未定义的数值");
        }
    }

    // 13. 所有的rust数值类型都支持和同类型之间的加减乘除取余运算
    #[allow(unused_variables)]
    {
        // 加法
        let sum = 5 + 10;

        // 减法
        let difference = 95.5 - 4.3;

        // 乘法
        let product = 4 * 30;

        // 除法
        let quotient = 56.7 / 32.2;

        // 求余
        let remainder = 43 % 5;

        // 浮点数一样可以取模
        let remainder = 43.0 % 5.0;
        println!("{}", remainder);
    }

    // 14. rust支持的位运算和c基本上一样 & | ^ ! << >>

    // 15. range rust提供的简洁语法生成连续的序列
    {
        // start..end -> [)
        // start..=end -> []
        let iter = 1..=5; // 是个迭代器
        for i in iter {
            println!("{}", i);
        }
        // 序列只允许用于数字或字符类型
    }

    
}
