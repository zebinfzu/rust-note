#![allow(unused)]

use std::fmt::Display;
fn main() {
    // 生命周期主要是为了避免悬垂引用
    /* {
        // rust编译器有借用检查器，以保证所有的借用都是有效的
        let r;         // -----------+ 'a
        {
            let x = 5; // -+-- 'b
            r = &x; // 报错 不能编译因为 r 引用的值在尝试使用之前就离开了作用域
        }              // -+
        println!("r: {}", r); // ----+
        // 编译器发现r的生命周期'a比x的生命周期'b大的多，因此不可以将x借用给r
    } */
    // 不会产生悬垂引用的正确例子
    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }
    // 函数中的泛型生命周期
    /* {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
        fn longest(x: &str, y: &str) -> &str { // 报错，不知道生命周期
            if x.len() > y.len() {
                x // x的生命周期
            } else {
                y // y的生命周期
            }
        }
    } */
    // 使用生命周期注释语法
    // 生命周期注解并不改变任何引用的生命周期的长短
    // 生命周期参数名称必须以撇号（'）开头，其名称通常全是小写
    // 函数签名中的生命周期注解
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
        // 使用注释语法，指明这两个参数和返回的引用存活的一样久
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        // println!("The longest string is {}", result); 报错，因为生命周期注释不会真的改变生命周期，只是会让函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致
    }
    // 指定生命周期参数的正确方式依赖函数实现的具体功能
    // 如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期
    {
        // 返回的引用值依赖于x，因此指明生命周期和参数x相关即可，此时不需要注释y的生命周期
        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }
    }
    // 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配
    // 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域
    // 这种情况就应该返回一个所有权数据而非引用，需要靠函数调用者去清理数据
    /* {
        // 报错
        fn longest<'a>(x: &str, y: &str) -> &'a str {
            let result = String::from("really long string");
            result.as_str()
        }
    } */
    // 存放引用的结构体需要生命周期注释
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    // 生命周期省略
    {
        // 编译器自己推的出来返回值该和哪个参数的生命周期挂钩，这时候可以省略不写生命周期注释
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }
    // 方法定义中的生命周期注释
    {
        // <>参数列表同时可以写泛型参数和生命周期注释，同时有的时候把生命周期注释写在前面
        struct ImportantExcerpt<'a, T> {
            part: &'a str,
            len: T,
        }
        impl<'a, T> ImportantExcerpt<'a, T> {
            fn level(&self) -> i32 {
                3
            }
        }
    }
    // 静态生命周期，存活于整个程序期间
    {
        let s: &'static str = "I have a static lifetime.";
    }
    // 结合生命周期、trait bounds和泛型参数
    {
        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
