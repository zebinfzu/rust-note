#[allow(unused)]
fn main() {
    // 1. Rust提供强大的模式匹配能力
    // 即将一个类型匹配该类型对应的值

    // 2. match
    {
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dire = Direction::South;
        // match必须是穷尽匹配的，即必须匹配该类型的所有值
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            }
            _ => println!("West"),
        };

        match 3_u8 {
            // 允许用range语法匹配一系列值，但这里不允许用..，只能用..=
            0..=99 => {
                println!("less than 100");
            }
            // 使用变量标识一类值，依照穷尽匹配，这里表示上一条所有没有匹配的值
            n => {
                println!("{n} large or equal than 100");
            }
        }

        match 100_i32 {
            // 当需要一个变量来表示一系列值的时候可以使用@绑定语法
            n @ -100..=0 => {
                println!("{}", n.abs());
            }
            // 使用一个元组表示该分支啥也不执行
            _ => (),
        }

        // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
        // match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
        // X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
    }

    // 3. match 模式绑定取出值
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
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
                // 将内部存储值绑定到了变量state上
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
    }

    // 4. if let 表达式
    {
        // 本质是match的语法糖，只有匹配的时候才执行，不需要穷尽匹配
        let n = Some(10);
        match n {
            Some(num) => println!("{num}"),
            None => (),
        }
        if let Some(num) = n {
            println!("{num}");
        }
    }

    // 5. matches!宏
    // 将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false
    {
        enum MyEnum {
            Foo,
            Bar,
        }
        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        // 如果要过滤，只保留MyEnum::Foo类型的值
        // v.iter().filter(|x| x == MyEnum::Foo); // 报错枚举成员不能使用==判断
        // v.iter().filter(|x| match x {
        //     MyEnum::Foo => true,
        //     _ => false
        // });
        // 使用matches!宏
        v.iter().filter(|x| matches!(x, MyEnum::Foo));

        let foo = 'f';
        assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

        let bar = Some(4);
        assert!(matches!(bar, Some(x) if x > 2));
    }
}
