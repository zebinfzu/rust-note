use std::ops::{Deref, DerefMut};

#[allow(unused)]
fn main() {
    // 1. Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
    {
        // 常规的解引用
        let x = 5;
        // y就是一个常规引用
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
        // 实现 Deref 后的智能指针结构体，就可以像普通引用一样，通过 * 进行解引用
        let x = Box::new(1);
        let sum = *x + 1;
    }

    // 2. 自定义智能指针并实现Deref
    {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }

    // 3. *背后其实是*(y.deref())先调用了deref方法获得一个常规的引用，然后*解除常规引用

    // 4. 函数和方法中的隐式Deref转换
    // 一个类型实现了Deref特征，那么引用传递给函数或者方法的时候会根据函数签名决定是否要做隐式的转换
    {
        fn display(s: &str) {
            println!("{}", s);
        }
        let s = String::from("hello world");
        display(&s);
        // String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
        // &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
        // 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)

        // Deref 可以支持连续的隐式转换，直到找到适合的形式为止
        let s = Box::new(String::from("hello world"));
        display(&s); // Box通过隐式转换成String，String再deref为&str
                     // 如果编译器不提供这种用隐式的转换就需要写作下面的形式
        display(&(*s)[..]);

        // 赋值的时候也会触发自动的deref
        let s = Box::new(String::from("str"));

        let s1: &str = &s;

        let s2 = s.to_string(); // Box上没有to_string方法，但是自动deref可以转换为&str，调到to_string方法
    }

    // 5. Deref规则总结
    {
        // 归一化
        // Rust 编译器只会对&v形式的引用做解引用操作，&&&&&v或者智能指针会做引用归一化转换为&v，然后再解引用
        // impl<T: ?Sized> Deref for &T {
        //     type Target = T;

        //     fn deref(&self) -> &T {
        //         // &self 是 self: &Self的简写 所以这里self是 &&T 需要 *&&T -> &T
        //         *self
        //     }
        // }
        // 按照上面的源码 &&&&v -> &&&v -> &&v -> &v 然后再deref
    }

    // 6. 三种Deref转换
    // 当 T: Deref<Target=U>，可以将 &T 转换成 &U
    // 当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
    // 当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
    {
        struct MyBox<T> {
            v: T,
        }
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox { v: x }
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.v
            }
        }

        // 要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.v
            }
        }
        // T: DerefMut<Target=U> 解读：将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，对应上例中，就是将 &mut MyBox<String> 转换为 &mut String
    }

    // 7. 表达式中需要手动解引用，不会触发deref
    {
        let a = Box::new(3);
        println!("a = {}", a); // 这是因为Box实现了Display，而不是自动触发解引用
    }
}
