fn main() {
    // 通过解引用运算符追踪指针的值
    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // 1. 像引用一样使用Box<T>
    {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // 2. 自定义智能指针
    {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        // 实现Deref特征使得rust知道如何像引用一样处理
        impl<T> std::ops::Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        // 调用*y -> 编译器理解为 *(y.deref()) -> *(&self.0)
        assert_eq!(5, *y);
    }

    // 3. 解引用强制转换，只能使用在实现了Deref特征的类型上面
    // 解引用强制转换将一种类型（A）隐式转换为另外一种类型（B）的引用，因为 A 类型实现了 Deref trait，并且其关联类型是 B 类型。比如，解引用强制转换可以将 &String 转换为 &str
    {
        // 标准库
        // #[stable(feature = "rust1", since = "1.0.0")]
        // impl std::ops::Deref for String {
        //     type Target = str;

        //     #[inline]
        //     fn deref(&self) -> &str {
        //         unsafe { str::from_utf8_unchecked(&self.vec) }
        //     }
        // }
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }
        let m = Box::new(String::from("Rust"));
        hello(&m);
    }
}
