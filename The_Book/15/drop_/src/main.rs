fn main() {
    // Drop 特征是智能指针的另外一个重要特性
    // 允许在值要离开作用域的时候执行一些代码
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    } // 离开作用域的时候会执行drop方法

    // 通过 std::mem::drop 提早丢弃值
    // 例如当使用智能指针管理锁时；你可能希望强制运行 drop 方法来释放锁以便作用域中的其他代码可以获取锁
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        // 下一行报错，这看着和别的语言的析构函数很像，但确实错误的，因为我们不可以禁止离开作用域时候自动的drop，因此如果这么做将会导致两次释放的问题
        // c.drop(); // 提前在离开作用域之前释放，这个是术语的析构函数
        std::mem::drop(c); // 这会告诉编译器在这里drop而不是离开之后drop
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
