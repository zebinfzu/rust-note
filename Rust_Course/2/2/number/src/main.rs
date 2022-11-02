#![allow(unused)]
fn main() {
    // 整数类型: i8 ~ isize u8 ~ usize
    {
        let _x: i8 = 0;
        let _x: i16 = 0;
        let _x: i32 = 0;
        let _x: i64 = 0;
        let _x: i128 = 0;
        let _x: isize = 0;
    }
    // 整数数值字面量
    {
        // 十进制
        let _x = 98_222;
        // 十六进制
        let _x = 0xffff;
        // 八进制
        let _x = 0o777;
        // 二进制
        let _x = 0b111000;
        // 字节，仅有u8可以使用
        let _x = b'A'; // b''数值对应于Ascii码值
        println!("{_x}");
    }
    // 整型溢出
    {
        // debug模式会检查整型溢出，当溢出的时候程序会panic，而release模式不会，会按照补码循环数字
    }
    // 浮点类型
    {
        // IEEE754标准实现，默认推导浮点数的字面量是f64
        let x: f32 = 2.0;
        let y: f64 = 8.8;
        // RUST里面浮点数是没有实现特征std::cmp::Eq，因此应该避免对浮点数做相等性测试，以及没有实现特征的原因，浮点型做不了HashMap的key
    }
    // 数学未定义
    {
        // 数学上未定义的值会返回NaN，该值和任何值互相操作都会返回NaN，不能用于比较
        let x = (-42.0_f32).sqrt();
        // 断定是不是NaN要用is_nan方法
        if x.is_nan() {
            println!("未定义的数学行为");
        }
    }
    // 数学运算
    {
        // 支持 + - * / %
        // rust有严格类型检查，不同类型做不了运算
        let x = 1 + 1;
        // let y = 1 + 1.0;
        // let x = 1u8 + 1i32;
        let x = 1 + 8.0 as i32;
    }
    // 位运算
    {
        // & | ^ ! << >>
    }
    // 序列 Range
    {
        // 数字..数字 生成左闭右开的序列，数字..=数字生成左闭右闭的序列
        // 序列只允许用于数字或字符类型，通常就是用在循环中
        for i in 1..5 {
            println!("{i}");
        }
    }
    // 复数
    {
        // rust复数不在标准库里面，需要在Cargo.toml中添加num包才能使用
        use num::complex::Complex;
        let a = Complex { re: 2.1, im: -1.2 };
        let b = Complex::new(11.1, 22.2);
        let result = a + b;

        println!("{} + {}i", result.re, result.im);
    }
}
