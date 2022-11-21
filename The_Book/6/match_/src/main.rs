#[allow(unused)]
fn main() {
    // 1. 枚举和模式匹配
    // rust提供极其强大的match能力
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }
    // 2. 枚举成员绑定到类型，match可以从成员中取出值进行匹配
    {
        #[derive(Debug)] // 这样可以立刻看到州的名称
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
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
    }
    // 3. rust的match必须是穷尽的，即匹配所有的可能性
    {
        let num = Some(10);
        match num {
            Some(1) => {
                println!("1");
            }
            Some(_) => {
                println!("!1");
            }
            None => {
                panic!("error");
            }
        }
    }
    // 4. match匹配项可以使用一个变量来占住任意的值，或者使用_表示任意该类的值，但不想使用(使用_的情况不会产生该变量)
    {
        let num = Some(10);
        match num {
            Some(t) => println!("{:?}", t),
            None => (),
        }
    }
    // 5. 当需要匹配一类值，值要通过序列语法生成的使用可以使用@语法糖来绑定变量
    // 同时当不想把所有情况写一遍的使用可以使用_做为最后的默认匹配项
    {
        let num = Some(11);
        match num {
            Some(a @ 0..=10) => println!("less than 11"),
            _ => ()
        }
    }

    // 6. if let 这是当我们不想穷尽匹配的使用可以使用的match语法糖
    {
        let num = Some(10);
        if let Some(t) = num {
            println!("{}", t);
        };
        // 等价于，但其实是为None时上面使用if let并不会执行
        match num {
            Some(t) => println!("{}", t),
            _ => ()
        }
    }
}
