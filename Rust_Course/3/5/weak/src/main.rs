use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
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

    // 3. 模拟真实情况下使用Weak解决循环引用
    {
        // 主人
        struct Owner {
            name: String,
            // 保存工具的时候不要存Rc而是存Weak，因为Weak不会增加引用计数，这样就不会导致循环引用
            gadgets: RefCell<Vec<Weak<Gadget>>>,
        }

        // 工具
        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
        }

        // 创建一个Owner
        // 注意主人可以拥有多个工具
        let gadget_owner: Rc<Owner> = Rc::new(Owner {
            name: "Gadget man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        });

        
        println!("创建两个工具之前，拥有者的引用计数是{}", Rc::strong_count(&gadget_owner));
        // 创建工具，并且与主人关联
        let gadget1 = Rc::new(Gadget {
            id: 1,
            owner: gadget_owner.clone(),
        });
        let gadget2 = Rc::new(Gadget {
            id: 2,
            owner: gadget_owner.clone(),
        });

        println!("创建两个工具之后，拥有者的引用计数是{}", Rc::strong_count(&gadget_owner));
        // 为主人更新所拥有的工具
        // 因为之前使用了Rc，所以要使用Weak，不然就会导致循环引用
        gadget_owner
            .gadgets
            .borrow_mut()
            .push(Rc::downgrade(&gadget1));
        gadget_owner
            .gadgets
            .borrow_mut()
            .push(Rc::downgrade(&gadget2));

        println!("将工具加入到所有者的工具数组字段后，工具的引用计数是{}", Rc::strong_count(&gadget1));
        // 遍历主人拥有的工具字段
        for gadget_opt in gadget_owner.gadgets.borrow().iter() {
            // gadget_opt是一个Weak<Gadget>
            // Weak不能保证数据一定存在，因此要显式调用upgrade()
            let gadget = gadget_opt.upgrade().unwrap();
            println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        }
    }
}
