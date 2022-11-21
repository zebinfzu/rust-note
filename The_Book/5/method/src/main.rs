#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// rust为一个类实现方法
impl Rectangle {
    // self 是语法糖
    // 方法的第一个参数可以是self，表示该方法获得调用者的所有权
    // &self，表示获得调用者的引用
    // &mut self，表示获得调用者的可变引用
    // 返回值同样有语法糖Self表示当前类型本身
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 第一个参数非self类的无法通过实例.语法调用，而是把类当作一个命名空间
    #[allow(unused)]
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::new(10, 40);

    println!(
        "The area of the rectangle is {} square pixels. {} square pixels",
        rect1.area(),
        rect2.area()
    );
}
