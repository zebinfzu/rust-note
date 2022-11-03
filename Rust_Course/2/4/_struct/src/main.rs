#![allow(unused)]
fn main() {
    // 结构体
    {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        // 创建实例
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        // 更新语法 注意，一样会发生所有权移动
        let user2 = User {
            active: false,
            ..user1
        };
        // 元组结构体
        // 字段没有名字，访问实例和访问元组一样使用.索引访问
        struct Color(u8, u8, u8);
        // 类单元结构体 主要用于不关心类型的内容，只关心行为的时候使用
        struct AlwaysEqual;
        // 结构体里面的字段要使用引用类型，就必须和生命周期一起用
        struct TT<'a> {
            username: &'a str
        }
        // 使用标识符#[derive(Debug)]来打印信息
    }
}
