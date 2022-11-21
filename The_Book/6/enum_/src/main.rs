#![allow(unused)]
fn main() {
    // 1. rust枚举
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }
        // 枚举值的类型相同，均是IpAddrKind
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        print!("four: {:?}, six: {:?}", four, six);
    }
    // 2. rust枚举的强大之处，枚举类型的不同值绑定到不同的类型
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        // 枚举成员绑定到不同类型的例子
        enum Message {
            Quit,                       // 没有任何关联类型
            Move { x: i32, y: i32 },    // 包含一个匿名结构体
            Write(String),              // 包含单独的一个String
            ChangeColor(i32, i32, i32), // (i32, i32, i32)的元组
        }
    }
    // 3. Option枚举，标准库提供，不需要Option前缀，可以直接使用Some和None
    {
        // enum Option<T> {
        //     Some(T),
        //     None,
        // }
        let some_number = Some(5);
        let some_string = Some("a string");
        // 当给的是None值，需要编译器Option的泛型
        let absent_number: Option<i32> = None;
    }
    // 4. rust的枚举在所有枚举成员不绑定值的时候，或者部分成员手动指定确定值时和c风格的枚举相同
    {
        // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
        enum Number {
            Zero,
            One,
            Two,
        }
        // 拥有显式辨别值（explicit discriminator）的 enum
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }
        // c风格的枚举可以被转换为整数
        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);

        println!("roses are #{:06x}", Color::Red as i32);
        println!("violets are #{:06x}", Color::Blue as i32);
    }
}
