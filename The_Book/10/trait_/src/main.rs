#![allow(unused)]

use std::{fmt::Display, iter::Sum};
fn main() {
    // 1. 通常面向对象的语言当中有接口类这一概念
    // 举例来说，猫可以call，狗可以call，假如我们不想给猫和狗都实现一个call方法，就可以实现一个抽象的动物类，没有具体的成员变量，但可以实现基类的call方法，子类继承该基类，可以复写父类的方法，子类不实现的话就会调用父类的方法

    // 2. 对于rust当中，没有继承的概念，那么如何实现类似java，c++中接口类的功能
    // 使用trait，特征是用来定义一系列方法的关键字，对于有同名方法的不同结构来说，可以为其单独实现trait
    {
        trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        // 一旦实现了trait，就可以调用trait中的方法
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    // 2. trait 可以提供默认实现
    {
        trait Mono {
            fn foo(&self) -> String {
                String::from("foo")
            }
        }

        struct A;

        impl Mono for A {}
        let a = A;
        println!("{}", a.foo())
    }

    // 3. 将trait绑定到参数，返回值
    {
        #[allow(dead_code)]

        trait Summary {
            fn summarize(&self) -> String;
        }

        // 这样的写法表示了一个缩写的泛型，item是一个泛型参数，类型可以是任意的实现了Summary特征的类型
        fn notify_0(item: impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }

        // 上面的写法等价于
        fn notify_1<T: Summary>(item: T) {
            println!("Breaking news! {}", item.summarize());
        }

        // trait绑定多个的写法
        fn notify_2<T: Summary + Copy>(item: T) {
            println!("Breaking news! {}", item.summarize());
        }

        // 使用where语句把trait绑定拆到函数签名后面
        fn notify_3<T, U>(i: T, j: U) -> ()
        where
            T: Summary + Copy,
            U: Clone + Display,
        {
        }

        // trait绑定到返回值
        fn notify_4<T>(i: T) -> impl Clone
        where
            T: Clone,
        {
            i
        }
    }

    // 4. 通过trait实现有条件的方法
    {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        // 实现有条件的方法，对泛型类型限制只有实现了特征的类型才有该方法
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
    }


    // 5. 通过trait绑定来对实现了trait的泛型实现特征
    {
        // impl<T: Display> ToString for T {
        //     // --snip--
        // }
        
    }
}
