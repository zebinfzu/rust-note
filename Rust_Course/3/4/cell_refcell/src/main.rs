use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[allow(unused)]
fn main() {
    // 1. Cell和RefCell提供了内部可变性，即不可变的变量但却可以修改内部的值
    // Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy
    {
        let c = Cell::new("asdf");
        // &str是实现了Copy特征的，因此在变量绑定的时候发生了copy
        let one = c.get();
        c.set("qwer");
        let two = c.get();
        println!("{}, {}", one, two); // asdf, qwer
    }

    // 2. RefCell
    {
        let s = RefCell::new(String::from("asdf"));
        // String没有实现Copy特征的
        // 编译期不会报错，但是运行报错
        // RefCell实现编译期可变、不可变引用共存 运行还是会报错
        // let s1 = s.borrow();
        // let s2 = s.borrow_mut();
        // println!("{},{}", s1, s2);
    }

    // 3. Rc 和 RefCell 组合使用
    {
        let s = Rc::new(RefCell::new("一个字符串".to_string()));
        let s1 = s.clone();
        let s2 = s.clone();
        // let mut s2 = s.borrow_mut();
        s2.borrow_mut().push_str("添加内容");
        println!("{:?}\n{:?}\n{:?}", s, s1, s2);
    }

    // 4. 内部可变性
    {
        // 对一个不可变的值进行可变借用，但这个并不符合 Rust 的基本借用规则
        let x = 5;
        // let y = &mut x; 编译器报错，禁止对不可变对象可变借用

        // 在某些场景中，一个值可以在其方法内部被修改，同时对于其它代码不可变，是很有用的
        {
            // 外部库中定义了一个消息发送器特征 Messenger
            // 它只有一个发送消息的功能：fn send(&self, msg: String)，因为发送消息不需要修改自身，因此原作者在定义时，使用了 &self 的不可变借用，这个无可厚非
            pub trait Messenger {
                fn send(&self, msg: String);
            }

            // --------------------------
            // 我们的代码中的数据结构和实现
            struct MsgQueue {
                msg_cache: Vec<String>,
            }
            // 在自己的代码中使用该特征实现一个异步消息队列，出于性能的考虑，消息先写到本地缓存(内存)中，然后批量发送出去
            //
            impl Messenger for MsgQueue {
                fn send(&self, msg: String) {
                    // self.msg_cache.push(msg)
                }
                // 该 send 方法的签名是 &self，因此上述代码会报错
            }
            // 使用内部可变性
            {
                pub struct MsgQueue {
                    msg_cache: RefCell<Vec<String>>,
                }
                impl Messenger for MsgQueue {
                    fn send(&self, msg: String) {
                        // 内部可变，这样外部库的特征中定义的函数签名的参数是不可变的借用，但实现却可以往不可变的借用中取到可变的借用
                        self.msg_cache.borrow_mut().push(msg)
                    }
                }
            }
        }
    }
}
