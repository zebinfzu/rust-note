fn main() {
    // 需求: 包含一个GUI结构的库
    // 这个 GUI 库包含一些可供开发者使用的类型，比如 Button 或 TextField
    // 在此之上用户可以自定义可以在上面绘制的图形

    // 在拥有继承的语言中，可以定义一个名为 Component 的类
    // 该类上有一个 draw 方法
    // 其他的类比如 Button、Image 和 SelectBox 会从 Component 派生并因此继承 draw 方法
    // 它们各自都可以覆盖 draw 方法来定义自己的行为，但是框架会把所有这些类型当作是 Component 的实例，并在其上调用 draw

    // 但是rust没有继承，因此要使用别的办法

    // trait对象
    // 定义通用的trait
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // screen的字段components是一个Vector，里面包含了指向了实现Draw特征对象的Box指针
        // 这与泛型不同，泛型+特征限制同时只能确定一类具体的类型
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    #[allow(unused, dead_code)]
    {
        // 一个screen实现了T就算是固定类型，components包含的也只能是固定类型了
        pub struct Screen<T: Draw> {
            pub components: Vec<T>,
        }

        impl<T> Screen<T>
        where
            T: Draw,
        {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
    }

    // 实现特征
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // 绘制实际的按钮
            println!("Draw a button");
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Draw a selectBox");
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // Trait 对象要求对象安全: 是对于trait来说的
    // 1. 该trait中的方法返回值类型必须不能是Self
    // 2. 方法没有任何泛型参数
    // 典型类型不安全的特征就比如Clone，clone方法的函数签名返回即是Self
}
