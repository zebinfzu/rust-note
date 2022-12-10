use std::fmt::{Debug, Display};

#[allow(unused)]
fn main() {
    // 1. rust的特征:定义一个可以被共享的行为，只要类型实现了特征就可以使用该行为
    // 类似其他语言的接口，实际干的是实现多态的活
    // 其他面向对象的语言，通常通过继承来实现父类子类有相同的方法或者子类复写父类的同名方法
    // 而rust当中实现这样的多态的效果就可以通过特征，特征提供基础实现，或者只提供函数签名，让类型实现特征的时候去获取基础实现或者决定要不要重写方法
    // 2. 定义特征
    // 使用trait关键字来声明一个特征，后面接特征名
    pub trait Summary {
        // 提供该特征需要实现的函数签名或者默认实现
        fn summarize(&self) -> String;
    }

    // 3. 为类型实现特征
    struct Post {
        title: String,
        author: String,
        content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }

    // 4. 调用特征方法
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    // 5. 特征与定义实现的位置遵循孤儿规则
    // 如果要为A类型实现特征T，那么A或者T至少要有一个是在当前作用域定义的

    // 6. 特征作为函数的参数
    {
        // 限制传入的参数必须实现了Summary特征类型实例的引用
        fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
    }
    // 面向对象语言通过继承来实现多态，为的是让其他函数可以通过一种基类来接收不同类型的参数，但却可以调用同一种方法
    // 比如类型猫和狗，都继承自基类动物，动物有方法say，猫和狗分别复写了这个方法
    // 这时候有一个函数接收一个动物类型的数组，函数内循环数组让动物的实例对象调用say方法
    // 对于rust则可以使用特征达到一样的效果，泛型函数限制传入对象均实现了Say特征
    {
        struct Cat; // 猫
        struct Dog; // 狗
        trait Say {
            fn say(&self) {
                println!("我是动物");
            }
        }
        impl Say for Cat {
            fn say(&self) {
                println!("我是猫");
            }
        }
        impl Say for Dog {
            fn say(&self) {
                println!("我是狗");
            }
        }
        // 一个函数，希望接收一个数组，这个数组均实现了say方法
        fn foo(arr: &Vec<Box<dyn Say>>) {
            for a in arr {
                a.say();
            }
        }
        // 对于rust是没办法直接存储不同类型的数组，因此需要通过特征对象
        // 即保存的是Box指针，指向的堆上对象必须实现了Say方法
        let arr: Vec<Box<dyn Say>> = vec![Box::new(Cat), Box::new(Dog)];
        foo(&arr);
    }

    // 7. 多重约束
    {
        fn notify_0(item: &(impl Summary + Display)) {}
        // 语法糖形式
        fn notify_1<T: Summary + Display>(item: &T) {}
        // 约束太多导致函数签名很难看懂的时候，可以使用where语法糖
        fn notify_2<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            1
        }
    }

    // 8. 使用特征约束有条件的实现方法
    {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }
        // 对于实现了Display和PartialOrd的泛型T，类型Pair<T>的实例才可以使用cmp_display方法
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
    }

    // 9. 指定返回值实现了特征，注意只能是一个类型
    {
        fn returns_summarizable() -> impl Summary {
            Weibo {
                username: String::from("sunface"),
                content: String::from("m1 max太厉害了，电脑再也不会卡"),
            }
        }
        // 如果在函数体内通过流程控制返回不同的类型编译会报错
        // fn returns_summarizable(switch: bool) -> impl Summary {
        //     if switch {
        //         Post {
        //             title: String::from("Penguins win the Stanley Cup Championship!"),
        //             author: String::from("Iceburgh"),
        //             content: String::from(
        //                 "The Pittsburgh Penguins once again are the best \
        //          hockey team in the NHL.",
        //             ),
        //         }
        //     } else {
        //         Weibo {
        //             username: String::from("horse_ebooks"),
        //             content: String::from("of course, as you probably already know, people"),
        //         }
        //     }
        // }
    }

    // 10. 通过derive派生trait
    {
        // derive宏可以帮助类型使用特征的默认实现
    }
}
