use std::{cell::RefCell, rc::Rc};
#[allow(unused)]
fn main() {
    // 1. rust同样会导致内存泄漏
    // 通过Rc和RefCell制造循环引用
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }
        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Self::Cons(_, item) => Some(item),
                    Self::Nil => None,
                }
            }
        }
        use List::*;
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("a指向的节点 = {:?}", a.tail());

        // 创建b到a的引用
        let b = Rc::new(Cons(10, RefCell::new(a.clone())));

        println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
        println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
        println!("b指向的节点 = {:?}", b.tail());
        /*
        // 利用RefCell的内部可变性，创建a到b的引用
        if let Some(link) = a.tail() {
            *link.borrow_mut() = b.clone(); // 好了，现在导致循环引用了
        }

        println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
        println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

        // 下面一行println!将导致循环引用
        // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
        // println!("a next item = {:?}", a.tail());
        */
    }

    // 2. Weak 类似 Rc，不同的是Rc持有所有权，Weak没有所有权，只保存一份指向数据的弱引用
    // 访问数据要通过upgrade方法，返回值是一个Option<Rc<T>>
    // 可访问，但没有所有权，不增加引用计数，因此不会影响被引用值的释放回收
    // 可由 Rc<T> 调用 downgrade 方法转换成 Weak<T>
    // Weak<T> 可使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 的值是 None
    // 常用于解决循环引用的问题
    {
        // 创建一个Rc，持有一个5
        let five = Rc::new(5);

        // 通过Rc创建一个Weak
        let weak_five = Rc::downgrade(&five);

        // Weak引用的资源依然存在，取到值5
        let strong_five: Option<Rc<_>> = weak_five.upgrade();
        assert_eq!(*strong_five.unwrap(), 5);

        // 手动释放five
        drop(five);

        // Weak引用资源不存在了，此时返回none
        let strong_five: Option<Rc<_>> = weak_five.upgrade();
        assert_eq!(strong_five, None);
    }
}
