#![allow(unused)]
// 定义结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    {
        // 创建实例
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
    }
    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");

        // ..结构体更新语法，用在最后指定还没有指定的字段
        // 更新语法一样对内部的内容遵循所有权的移动，克隆，拷贝原则
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
        // 到这里user1.username不可以使用了，因为被move给user2.username，user1.active还可以正常使用，因为栈上数据发生的是copy
    }

    {
        // 元组结构体
        struct Color(i32, i32, i32);
        let black = Color(0, 0, 0);
        // 使用.语法索引元组结构体的内容
        println!("{}", black.0);
    }
    {
        // 没有任何字段的类单元结构体
        struct AlwaysEqual;
        let subject = AlwaysEqual;
    }
    {
        // 结构体存储的字段类型必须拥有数据的所有权，如果要存储引用必须得带上生命周期
        // struct User {
        //     active: bool,
        //     username: &str,
        //     email: &str,
        //     sign_in_count: u64,
        // }
    }
}

// 初始化字段简写语法
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
