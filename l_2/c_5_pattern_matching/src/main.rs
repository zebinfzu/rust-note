#![allow(unused)]
/** match的通用形式
 match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
*/
fn foo1() {
    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;
    // match和别的语言的switch类似，_差不多相当于default
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };
}

fn foo2() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // match后面跟的是一个表达式，if跟的表达式必须是布尔值，match则可以是任何值
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn foo3() {
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    // match本身也是表达式，可以用于赋值
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}

fn foo4() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --skip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // 25美分硬币
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    value_in_cents(Coin::Quarter(UsState::Alabama));
}

fn foo5() {
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }
    let actions = [
        Action::Say("Hello Rust".to_string());
        Action::MoveTo(1, 2);
        Action::ChangeColorRGB(255, 255, 0);
    ]
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

fn foo6() {
    // match必须穷尽匹配，否则编译器报错
    // 如果不想写完所有情况，比如值很多，但只有这个是关心的情况下，就可以使用_通配符
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn foo7() {
    // 只有一个模式的值需要处理时match很啰嗦
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
    // 这时候可以使用if let
    if let Some(3) = v {
        println!("three");
    }
}

fn foo8() {
    enum MyEnum {
        Foo,
        Bar
    }
        
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    // 如果要对v进行过滤
    v.iter().filter(|x| x == MyEnum::Foo); // 会报错，因为无法将x直接和一个枚举成员比较
    
    v.iter().filter(|x| matches!(x, MyEnum::Foo));
}


fn main() {
    foo4();
}
