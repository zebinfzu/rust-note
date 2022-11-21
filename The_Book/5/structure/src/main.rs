#![allow(unused)]
fn main() {
    // 1. rust 中使用struct 定义结构体，其中每一部分数据的名字和类型称为字段(field)
    {
        // 自动实现Debug特征，就可以使用{:?}来打印
        // 结构体内字段同样遵循rust当中的所有权规则，即copy,move,clone
        // Rust 并不允许只将某个字段标记为可变,必须是整个对象可变
        #[derive(Debug)]
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        // 创建一个实例
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        println!("{:?}", user1);

        // 结构体更新语法: 表示剩下字段来源于指定对象
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
        // 这时user1中的username字段类型为String,发生了移动，如果再次通过user1访问username会导致报错
    }
    // 2. 元组结构体和类单元结构体
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        // 通过.索引来访问
        println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);

        // 类单元结构体
        struct AlwaysEqual;
    }
}
