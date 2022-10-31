#![allow(unused)]

use std::fmt::format;
fn main() {
    {
        // rust语言本身字符串类型：字符串slice str，通常以借用的形式出现&str
        // String是标准库提供的，不是核心语言的内容
        // str,String均是utf-8
    }
    {
        // 创建String
        let mut s = String::new();
        // 字符串字面值，是字符串slice的引用
        let data = "initial contents";
        let s = data.to_string();
        // 该方法也可直接用于字符串字面值：
        let s = "initial contents".to_string();
        let s = String::from("initial contents");
    }
    {
        // 更新字符串
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{s}");

        // push_str并没有获取参数的所有权
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        // 之后s2任然可以使用
        println!("s2 is {}", s2);
        // push方法的参数是单个char
        let mut s = String::from("lo");
        s.push('l');
    }
    {
        // 字符串加法
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

        // 实际上+调用的是String本身的方法add(self, s: &self) -> String，因此s1左右操作符的左边调用方法，嫩是的所有权被移交给了add函数
        // 通过add方法可以知道，只能让String和&str相加，而不能让String + String
        // 这样的设计是为了减少拷贝，移动了s1的所有权，然后使用s2的借用，将s2的内容拷贝到s1的后面，再把所有权返回去

        // 串联多个字符串
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        // 1. 使用+
        // let s = s1 + "-" + &s2 + "-" + &s3;
        // 2. 使用format宏 这个宏和println差不多，不过不输出到屏幕而是返回一个String的所有权，同时不会获取任何参数的所有权
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{s1} {s2} {s3} {s}");
    }
    {
        // RUST中不能使用下标索引字符串
        let s1 = String::from("hello");
        // let h = s1[0]; 报错，类型String不能使用索引
        // 这是因为，utf-8存储的时候每个字符不是固定的字节数，因此使用索引访问就很容易出错

        // 遍历字符串的方法
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
