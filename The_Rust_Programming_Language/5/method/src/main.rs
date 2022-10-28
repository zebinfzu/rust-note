#![allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块中定义的函数被称为关联函数
impl Rectangle {
    // &self 是语法糖 self: &Self的缩写
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 方法可以和字段名重叠
    fn width(&self) -> u32 {
        self.width
    }
    // 可以定义一个不以self作为第一个参数的关联函数，因为不需要绑定到具体实例，因此使用::来使用，通常定义new函数来作为返回一个结构体新实例的构造函数
    // Self代表impl之后的类型
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 40);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
