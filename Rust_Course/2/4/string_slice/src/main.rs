#![allow(unused)]
fn main() {
    // 切片
    {
        // 创建语法 集合[开始索引..结束索引]
        let s = String::from("abcdefg");
        // 不要用range创建字符串切片，因为每个字符的字节长度不一定相等
        let slice = &s[..]; // 引用整个String
        // 字符串字面量是切片
        let s = "hello";
        // 其他集合一样有切片，例如数组
        let a = [1, 2, 3, 4, 5];
        // 切片类型是&[t]
        let b = &a[1..3];
    }
    // 什么是字符串
    {
        // 如何将String转换到&str
        // 取引用
        // 不要创建
        let s = String::from("kkkkk");
        let a = &s;
        let b = &s[..];
        let c = s.as_str();
        // mut String 的一些常用方法
        // push 追加
        // insert
        // replace replacen replace_range
        // delete
        // 链接字符串使用 + += 左操作数是String，右操作数是&str，该方法会移动左操作数的所有权
        // format!宏链接字符串，该宏不会拿走变量的所有权
    }
}
