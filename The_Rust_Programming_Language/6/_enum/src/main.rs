#![allow(unused)]

fn main() {
    {
        // 枚举值位于标识符的命名空间中
        enum IpAddrKind {
            V4,
            V6,
        }
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        fn route(ip_kind: IpAddrKind) {}
        route(IpAddrKind::V4);
        route(six);
    }
    {
        // 上一个代码块只知道类型而不知道具体的值
        // 使用结构体来存储值
        enum IpAddrKind {
            V4,
            V6,
        }
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }
    {
        // 利用枚举更加简单的表示上一个代码块相同的概念\
        // 枚举成员的名字变成了一个构建枚举实力的函数
        enum IpAddr {
            V4(String), // IpAddr::V4是一个获取String参数并返回IpAddr类型实力的函数调用
            V6(String),
        }
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
    }
    {
        // 利用枚举来替代结构体的另外一个优势
        // 结构体的每个成员数据类型必须是相同的
        // 枚举则可以每个成员处理不同的类型和数量的数据
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }
    {
        // 标准库是如何定义 IpAddr
        // 可以将任意类型的数据放入枚举成员中
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }
    {
        // Quit:没有任何关联数据
        // Move:类似结构体包含命名字段
        // Write 包含一个单独的String
        // ChangeColor 包含三个i32
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        // 对于所有有关联值的枚举的方式和定义多个不同类型的结构体很相似
        // 但是我们使用不同的结构体，他们的类型是不同的，没办法轻易的定义一个能够处理这些不同类型的结构体函数
        struct QuitMessage; // 类单元结构体
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // 元组结构体
        struct ChangeColorMessage(i32, i32, i32); // 元组结构体

        // 枚举是一个类型，因此可以为枚举实现方法
        impl Message {
            fn call(&self) {}
        }
        let m = Message::Write(String::from("hello"));
        m.call();
    }
    {
        // 标准库当中常用枚举Option
        // 标准库中定义如下，甚至直接包含在prelude中，不需要显示引入
        // enum Option<T> {
        //     None,
        //     Some(T),
        // }
        let some_number = Some(5);
        let some_char = Some('e');
        let absent_number: Option<i32> = None;

        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        // 报错，i8 和 Option<i8>类型不同，不能相加
        //let sum = x + y;
        // i8 这样类型的值时，编译器确保它总是有一个有效的值，不需要做空值检查

        // 只有当使用 Option<i8>（或者任何用到的类型）的时候需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况
    }
}
