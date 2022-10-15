struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    // Cricle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // &self 是 self: &Self的简写，Self代表被实现该方法的结构体类型，self代指此类型的实例
    // self依旧有所有权的概念
    // self表示实力所有权转移到该方法当中，比较少用
    // &self表示该方法对实例的不可变借用
    // &mut self表示可变借用
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// rust当中允许方法名和结构体字段名相同
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

// 一般来说，方法名和字段同名，适合用来实现getter访问器
pub struct Tt {
    width: u32,
    height: u32,
}

impl Tt {
    // new 在rust里面不是关键字，因为第一个参数约定俗成不是self
    // 因此new是函数而不是方法，不能使用.调用，需要使用类名::new()调用，因为该方法处于结构体的命名空间当中，使用::语法调用命名空间里面的内容
    pub fn new(width: u32, height: u32) -> Self {
        Tt { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
}

fn foo() {
    // 枚举类型实现方法
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // 这里定义方法体
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
