use std::{cell::RefCell, rc::Rc};

#[allow(unused)]
fn main() {
    // interior mutability 是 rust 的一个设计模式
    // 允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的
    // 该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则
    // 所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的
    // 不同于 Rc<T>，RefCell<T> 代表其数据的唯一的所有权

    // 回顾借用的规则
    // 1. 任意时刻，只可能有可变引用或者不可变引用的一种
    // 2. 可变引用任意时候至多一个，不可变引用则无此限制
    {
        let mut a = 10;
        let b = &mut a;
        *b = 10;
        let mut a = 10;
        let b = &a;
        let c = &mut a;
        // println!("{}", b); // 报错，在申明了c之后又使用b，把b的lifetime延长到c之后是不合法的

        let x = 5;
        // let y = &mut x; // 报错，因为不可变的x不能获取到可变的借用
    }

    // 对于RefCell，一样只用在单线程，如果在多线程上下文中使用将会导致编译错误

    // RefCell可以使得不可以的数据内部可变
    {
        let v = RefCell::new(vec![0]); // 不可变对象

        // 但是由于RefCell内部使用了Unsafe来模糊
        // 所以允许
        v.borrow_mut().push(1);

        // RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一

        let mut v_1 = v.borrow_mut();
        // 同一作用域同时创建了两个可变引用
        // 编译器不对RefCell这种情况报错，而是运行时panic
        // let mut v_2 = v.borrow_mut();
    }

    // 利用RefCell和Rc来创建多个可变数据拥有者
    {
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        // 所谓多个拥有的意思就是，b,c都是可变的，但不代表同一个时刻可以多个可变去改变值
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(12)), Rc::clone(&a));

        // 解除可变引用+10
        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    // 利用RefCell和Rc来强制制造循环引用导致内存泄漏
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }
        use List::{Cons, Nil};
        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); // a的下一个节点是空节点
        // Rc(Crons(5, RcCell(Rc(Nil))))
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        // Rc(Crons(10, RcCell(a)))
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());
        if let Some(link) = a.tail() {
            println!("{:?}", link); // 拿到a的下一个节点是RefCell Nil
            let mut f = link.borrow_mut(); 
            *f = Rc::clone(&b); // a -> b 导致无限的循环，寄
            // a -> Rc(Crons(5, ReCell(b)))
        }
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail()); // 无限的循环链表
    }

    // 创建引用循环并不容易，但也不是不可能
    // 如果你有包含 Rc<T> 的 RefCell<T> 值或类似的嵌套结合了内部可变性和引用计数的类型，请务必小心确保你没有形成一个引用循环

    // 另一个解决方案是重新组织数据结构，使得一部分引用拥有所有权而另一部分没有
}
