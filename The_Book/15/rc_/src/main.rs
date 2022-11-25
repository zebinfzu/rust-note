#![allow(unused)]

use std::rc::Rc;
fn main() {
    // 引用计数Rc
    // 这个智能指针用在一个值希望有多个所有者的情况下
    // 注意Rc只能用在单独的线程
    // 常见的情况比如有两个单向的链表，在一个节点交汇，后面的节点共用，因此不能一条不需要使用了就把后面节点给删了
    {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        use List::{Cons, Nil};
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        // 下一行报错，提示a已经move了
        // let c = Cons(4, Box::new(a));
    }

    // 修改List的定义，使用Rc
    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        use List::{Cons, Nil};
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
        // Rc::clone不像其他的类型的clone方法通常是深拷贝，而是会增加引用计数
        {
            // 利用嵌套作用域看看引用计数的变化
            {
                let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
                // strong_count方法可以获取当前的引用计数
                println!("count after creating a = {}", Rc::strong_count(&a));
                let b = Cons(3, Rc::clone(&a));
                println!("count after creating b = {}", Rc::strong_count(&a));
                {
                    let c = Cons(4, Rc::clone(&a));
                    println!("count after creating c = {}", Rc::strong_count(&a));
                }
                println!("count after c goes out of scope = {}", Rc::strong_count(&a));
            } // 到这里引用计数清零了才会drop
              // 通过不可变引用， Rc<T> 允许在程序的多个部分之间只读地共享数据
              // 如果 Rc<T> 也允许多个可变引用，则会违反第 4 章讨论的借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致
        }
    }
}
