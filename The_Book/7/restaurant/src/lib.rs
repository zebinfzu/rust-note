#![allow(unused, dead_code)]
/*
模块树的结构
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// pub mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// // mod中的路径规则
// pub fn eat_at_restaurant() {
//     // 绝对路径，从crate开始
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径，从当前文件的层级找
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // super语法进入当前mod的父目录
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// 使用use关键字引入作用域
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// 使用pub use重新导出名称
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 重新导出之后，外部模块可以使用restaurant::hosting::add_to_waitlist路径来引入add_to_waitlist函数
// 否则需要使用restaurant::front_of_house::hosting::add_to_waitlist
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// extern 关键字用来导入其他crate
extern crate std as ruststd;

use ruststd::collections::HashMap;
