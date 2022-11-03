#![allow(unused)]
fn main() {
    // 所有的模式语法

    // 1. 匹配字面值
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // 2. 匹配命名变量
    {
        let x = Some(5);
        let y = 10;
        // match会创造一个新的作用域
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y), // 这里引入了一个新的变量y，会匹配任何Some的值，一旦match表达式作用域结束了，y也结束
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }
    // 匹配多个模式 |
    {
        let x = 1;
        // match表达式里面使用|语法表示想要匹配多个值
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    // 匹配范围值 ..=
    {
        let x = 5;
        // range语法只能用在数字和char
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }
    // 解构并赋值
    {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        // 结构体解构并重命名
        let Point { x: a, y: b } = p;
        let p = Point { x: 10, y: 0 };
        match p {
            // x匹配任何值，y匹配0，之后表达式可以使用x,y
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }

        // 解构枚举
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }

        // 解构嵌套结构体和枚举
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum MessageV2 {
            Quit,
            ChangeColor(Color),
        }
        let msg = MessageV2::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            MessageV2::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            MessageV2::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
            _ => (),
        }
    }
    // 使用 _ 忽略值
    {
        // 嵌套使用
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 8, 16, 32);
        // 忽略掉不想使用的，同时_所有权不会被移入
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }

        let s = Some(String::from("Hello!"));
        // 这里如果不使用_则Some里面的String所有权会被移入，导致之后不能继续使用s
        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);

        // 使用..忽略剩余值
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        // 使用..扩展到需要的数量
        let numbers = (2, 4, 8, 16, 32);
        // .. 必须要是无歧义的，.. some .. 这种过不了编译
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }
    // 匹配守卫(match guard)提供额外的条件，使用一个if表达式
    {
        let num = Some(5);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }

        // 匹配守卫用来解决变量覆盖的问题
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => (),
            // 匹配守卫不会覆盖变量名
            Some(n) if n == y => (),
            _ => (),
        }
    }

    // @运算符
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };
        // 在创建一个存放值的变量的同时测试其值是否匹配模式
        match msg {
            Message::Hello {
                id: id_variable @ 3..=7, // 创建变量的时候测试值是否处于3..=7之间
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
