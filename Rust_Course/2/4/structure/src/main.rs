#[allow(unused)]
fn main() {
    // 1. 结构体语法
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // 2. 结构体实例
    {
        // 创建实例
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        // 通过.访问字段
        println!("{}", user1.username);

        // 无法设置单独字段的可变性，若需要可变则整个实例对象可变
        let mut user2 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        user2.email = String::from("anotheremail@example.com");
    }
    // 3. 解构体更新语法
    {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        // 结构体更新语法，创建实例的时候可以用同类型的实例来更新剩余没写的字段
        let user2 = User {
            active: false,
            ..user1
        };
        // 注意字段依旧遵从所有权规则，此时user1的email，username字段已经发生了move，不可以再使用了
    }
    // 4. 元组结构体
    {
        // 字段没有名字，看着很像元组
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
    // 5. 单元结构体
    {
        // 没有内容，但有行为
        struct AlwaysEqual;
    }
    // 6. 结构体中的字段是引用的时候需要给定生命周期标识符
    {
        struct User<'a> {
            username: &'a str,
            email: &'a str,
            sign_in_count: u64,
            active: bool,
        }
    }
    // 7. Debug特征可以让println!({:?}, arg)接收struct参数
    {
        // 通过derive自动实现
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("rect1 is {:?}", rect1);
    }

    // 8. 通过Debug宏dbg!，这个宏输出到stderr，会获取输入表达式的所有权
    // 输出代码所在的文件名、行号、表达式以及表达式的值
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }
}
