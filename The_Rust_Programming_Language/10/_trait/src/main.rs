#![allow(unused)]

use std::fmt::{Debug, Display};
fn main() {
    {
        // 定义trait，rust中的trait有点像别的语言里面interface的概念，但又不完全相同
        // 对不同类型调用相同的方法的话，这些类型就可以共享相同的行为
        // trait 定义是一种将方法签名组合起来的方法
        // 目的是定义一个实现某些目的所必需的行为的集合
        // 一行一个方法签名且都以分号结尾
        pub trait Summary {
            fn summarize(&self) -> String;
        }
    }
    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        // 为类型实现trait
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
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }
    {
        // 只有trait或者类型至少其中一者的定义处于你的crate当中时，才允许为类型实现trait，这条规则确保了其他人编写的代码不会破坏你代码
    }
    {
        // 默认实现
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        // 使用trait作为参数
        // 表示item是实现了Summary的任意类型
        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        // Trait Bound语法糖
        pub fn _notify<T: Summary>(item: T) {
            println!("Breaking news! {}", item.summarize());
        }
        // 通过+指定同时实现了多个trait
        pub fn _notify_<T: Summary + Display>(item: &T) {}
        // pub fn notify(item: &(impl Summary + Display)) {}

        // 通过 where 简化 trait bound
        fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
        // where可以写在返回参数后面，主要是为了让签名更好看懂
        fn _some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            1
        }
    }
    {
        // 返回值指定返回类型实现了trait
        // fn returns_display() -> impl Display {

        // }
    }
    {
        // 通过实现trait来限制泛型，修复largest函数
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }
    }
    {
        // 通过trait bound实现有条件的方法
        struct Pair<T> {
            x: T,
            y: T,
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }
        // 有条件的方法
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
}
