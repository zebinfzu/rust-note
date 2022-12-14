use std::{rc::Rc, sync::Arc, thread};

#[allow(unused)]
fn main() {
    // 1. Rc 单线程的引用计数、Arc 多线程的引用计数
    // 2. rust所有权机制要求一个值只能有一个所有者，但是这会导致图数据机构多个边拥有同一个点的问题，还有诸如多线程可能会持有同一个数据

    // 3. Rc
    // 分配一个堆上的对象供给多个部分使用，直到使用该对象的变量全挂了才会挂
    {
        let s = String::from("str");
        // a 赋值的时候已经拿走了s的所有权
        // let a = Box::new(s);
        // s 不能在被使用
        // let b = Box::new(s);

        // 使用Rc
        let a = Rc::new(String::from("str"));
        let b = Rc::clone(&a); // 或者a.clone() 内部指向同一个string
    }

    // 4. Rc中计数的变化
    {
        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    // 5. Rc<T> 是指向底层数据的不可变的引用，需要修改内部值要和内部可变性RefCell<T>连用或者多线程使用Arc和Mutex<T>

    // 6. 一个Rc的例子
    {
        struct Owner {
            name: String,
        }
        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
        }

        // 创建一个基于Rc的Owner
        let gadget_owner = Rc::new(Owner {
            name: "Gadget man".to_string(),
        });

        // 创建两个工具，属于同一个主人
        let gadget1 = Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        };
        let gadget2 = Gadget {
            id: 2,
            owner: Rc::clone(&gadget_owner),
        };

        // 释放掉第一个 `Rc<Owner>`
        drop(gadget_owner);
        // 但是gadget_owner的内容还是可以被使用
        // 而且Rc是一个实现了deref的智能指针，所以不需要解引用，直接就是可以gadget1.owner.name
        println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
        println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

        // 在作用域最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
        // 数据也被清理释放
    }

    // 7. Arc 就是原子化(Atomic)的Rc，用于多线程
    // 保证数据能够安全的在线程间共享
    {
        let s = Arc::new(String::from("多线程可用"));
        for _ in 0..10 {
            let s = Arc::clone(&s);
            let handle = thread::spawn(move || {
                println!("{}", s);
            });
        }
    }
}
