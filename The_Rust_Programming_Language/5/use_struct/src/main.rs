#![allow(unused)]
fn main() {
    // rectangles
    {
        let width = 30;
        let height = 40;
        fn area(width: i32, height: i32) -> i32 {
            width * height
        }
        area(width, height);
    }
    {
        // 让代码更具有可读性，使用元组重构入参
        fn area(dimensions: (i32, i32)) -> i32 {
            dimensions.0 * dimensions.1
        }
    }
    {
        // 使用结构体重构
        // 增加外部属性来派生Debug Trait
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
        println!("rect1 is {:?}", rect1);
    }
    
}
