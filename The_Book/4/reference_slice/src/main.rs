fn main() {
    // 1. &运算符获取一个引用，*解引用，通过mut标识引用可变性
    {
        let mut x = 5;
        let y: &mut i32 = &mut x;
        *y += 10;
        println!("{}", x);
    }
    // 2. 或者声明变量的时候使用ref标识赋值得到的是右边的引用
    {
        let x = 5;
        let ref y = x;
        println!("{}", y);
    }
    // 3. 不可以同时创建可变引用和不可变引用
    // 4. 可变引用同时只能有一个
    // 5. 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止

    // 6. slice 和引用一样是对数据没有所有权的类型，通常为&[t]
    // 字符串切片&str
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        println!("{} {}", hello, world);
    }
}
