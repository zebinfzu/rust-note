#![allow(unused)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 过不了编译 因为hosting模块私有
// rust当中默认所有项(函数、方法、结构体、枚举、常量)都是私有的
// 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项
/* pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
} */

pub fn take_order_at_restaurant() {
    // 绝对路径
    crate::front_of_house::serving::take_order();

    // 相对路径 相当于从当前作用域相同的位置
    front_of_house::serving::take_order();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super 从父目录开始寻找 这里back_of_house的父模块就是lib.rs，也就是根crate
        super::serve_order();
    }

    fn cook_order() {}

    // 创建公有枚举，同样字段任然默认是私有的，需要pub关键字
    pub struct Breakfast {
        pub toast: String, // 公有字段，外界可以访问
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 对于枚举，要把所有成员变成公有的，则只需要给枚举加上pub就可以
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant_2() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// 使用use关键字将路径引入作用域
use crate::front_of_house::serving;

pub fn eat_at_restaurant_3() {
    serving::take_order();
}

// 使用pub use导出
pub use crate::front_of_house::hosting;
// 外部代码可以通过新路径restaurant::hosting::add_to_waitlist来使用函数
// 否则只能使用restaurant::front_of_house::hosting::add_to_waitlist

// 使用路径嵌套来消除多余的use行
// use std::cmp::Ordering;
// use std::io;
// 改为
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// 改为
// use std::io::{self, Write};

// 通过glob运算符引入所有的公有定义
use std::collections::*;
