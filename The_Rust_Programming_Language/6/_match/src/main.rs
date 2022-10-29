#![allow(unused)]
fn main() {
    // Rust 有一个叫做 match 的极为强大的控制流运算符
    // 允许将一个值与一系列的模式相比较
    // 并根据相匹配的模式执行相应代码
    // 模式可由字面值、变量、通配符和许多其他内容构成
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
    {
        // 绑定值的模式
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
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
                    // state会绑定Quarter传入的值
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
    }
    {
        // 匹配Option<T>
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
    {
        // 通配模式和_占位符
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            // other是一个特殊的变量，表示所有没有列出的值
            other => move_player(other),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            // 对比other，不想在处理时绑定值的话可以使用_
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }
    {
        // 当match到不需要的值的时候，返回空元组来表示不想运行任何代码
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
    }
}
