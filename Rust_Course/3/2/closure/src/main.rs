#[allow(unused)]
fn main() {
    // 1. rust闭包是一种匿名函数，可以被赋值给变量也可以作为参数传递给其他函数，和一般函数不同的是，允许捕获调用者作用域中的值
    {
        let x = 1;
        let sum = |y| x + y;
        assert_eq!(3, sum(2));
    }
    // 2. 闭包的例子
    {
        // 需求：实现一个健身函数，接收一个强度值，一个随机值
        use std::thread;
        use std::time::Duration;

        fn muuuuu(intensity: u32) -> u32 {
            println!("muuuu.....");
            thread::sleep(Duration::from_secs(2));
            intensity
        }
        {
            fn workout(intensity: u32, random_number: u32) {
                if intensity < 25 {
                    println!("今天活力满满，先做 {} 个俯卧撑!", muuuuu(intensity));
                    println!(
                        "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
                        muuuuu(intensity)
                    );
                } else if random_number == 3 {
                    println!("昨天练过度了，今天还是休息下吧！");
                } else {
                    println!(
                        "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                        muuuuu(intensity)
                    );
                }
            }
        }

        // 上面的函数workout多处调用了muuuuu函数，如果不再用这个函数了，需要替换的位置很多

        // 使用函数变量来存储
        {
            fn workout(intensity: u32, random_number: u32) {
                // 这样要替换执行的函数名的时候，只要替换一个位置就可以了
                let action = muuuuu;
                if intensity < 25 {
                    println!("今天活力满满, 先做 {} 个俯卧撑!", action(intensity));
                    println!(
                        "旁边有妹子在看，俯卧撑太low, 再来 {} 组卧推!",
                        action(intensity)
                    );
                } else if random_number == 3 {
                    println!("昨天练过度了，今天还是休息下吧！");
                } else {
                    println!(
                        "昨天练过度了，今天干干有氧, 跑步 {} 分钟!",
                        action(intensity)
                    );
                }
            }
        }

        // 但是依赖于intensity的变化还是要修改多处，因此可以使用闭包来捕获当前作用域中的变量
        {
            fn workout(intensity: u32, random_number: u32) {
                let action = || {
                    println!("muuuu.....");
                    thread::sleep(Duration::from_secs(2));
                    intensity
                };

                if intensity < 25 {
                    println!("今天活力满满，先做 {} 个俯卧撑!", action());
                    println!("旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!", action());
                } else if random_number == 3 {
                    println!("昨天练过度了，今天还是休息下吧！");
                } else {
                    println!("昨天练过度了，今天干干有氧，跑步 {} 分钟!", action());
                }
            }
        }
    }

    // 3. 闭包结构
    {
        // 需求：实现一个具有缓存功能的结构体
        // T是泛型参数，Fn(u32) -> u32是T的特征约束，表明T是一个闭包类型
        // 特征 Fn(u32) -> u32 从表面来看，就对闭包形式进行了显而易见的限制：该闭包拥有一个u32类型的参数，同时返回一个u32类型的值
        struct Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            query: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(query: T) -> Cacher<T> {
                Self { query, value: None }
            }

            // 先查询缓存值是否存在，不存在就调用query加载
            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.query)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }
    }

    // 4. 三种Fn特征
    {
        // FnOnce 运行闭包的时候，捕获的值会拿走所有权，因此只能运行一次
        fn fn_once<F>(func: F)
        where
            F: FnOnce(usize) -> bool,
        {
            println!("{}", func(3));
            // println!("{}", func(4)); 报错，FnOnce特征表示该闭包只能运行一次
        }
        let x = vec![1, 2, 3];
        fn_once(|z| z == x.len()); // x被move到闭包里面
        println!("{:?}", x);

        // 但是可以给约束条件加上Copy特征，这样调用的时候就是调用的Copy的闭包，可以调用多次
        fn fn_once_<F>(func: F)
        where
            F: FnOnce(usize) -> bool + Copy, // 改动在这里
        {
            println!("{}", func(3));
            println!("{}", func(4));
        }

        // 如果想要强制闭包获得捕获变量的所有权，可以在参数列表前面加上move
        use std::thread;
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));
        // 让另外一个线程获取当前线程的数据所有权
        handle.join().unwrap();

        // FnMut 获取可变借用，可以修改值
        let mut s = String::new();
        // 想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型
        // 这里和move比起来比较反直觉
        let mut update_string = |str| s.push_str(str);
        update_string("hello");

        println!("{:?}", s);
        // 当作普通变量，在函数定义的时候明确指明
        {
            fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
                f("hello");
            }
            let mut s = String::new();
            let update_string = |str| s.push_str(str);
            exec(update_string);
            println!("{:?}", s);
        }

        // Fn 特征，它以不可变借用的方式捕获环境中的值
    }

    // 5. move 和 Fn
    {
        // 一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们
        fn exec<F: FnOnce()>(f: F) {
            f()
        }
        let s = String::from("hello");
        // 这里move了，闭包不仅实现了FnOnce，也实现了特征Fn
        let update_string = move || println!("{}", s);

        exec(update_string);
    }

    // 6. 三种Fn特征的关系
    // 所有闭包自动实现FnOnce特征，因此至少可以调用一次
    // 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    // 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
    {
        let s = String::new();
        // 闭包中只是使用了s的不可变借用
        let update_string = || println!("{}", s);
        // 因此可以适用于任何一种 Fn 特征
        exec(update_string);
        exec1(update_string);
        exec2(update_string);
        fn exec<F: FnOnce()>(f: F) {
            f()
        }
        fn exec1<F: FnMut()>(mut f: F) {
            f()
        }
        fn exec2<F: Fn()>(f: F) {
            f()
        }
        // 一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们，跟是否使用 move 没有必然联系
    }

    // 7. 闭包作为函数的返回值
    {
        // rust的函数要求返回值有固定大小的确定类型
        // 特征类似接口，编译期不知道大小，因此没法直接返回
        fn factory() -> impl Fn(i32) -> i32 {
            let num = 5;
            move |x| x + num
        }
        let f = factory();
        let answer = f(1);
        assert_eq!(6, answer);

        // 但是impl的方式返回特征只能是固定的类型，如果要返回不同类型还是需要特征对象
        fn factory_(x: i32) -> Box<dyn Fn(i32) -> i32> {
            let num = 5;
            if x > 1 {
                Box::new(move |x| x + num)
            } else {
                Box::new(move |x| x - num)
            }
        }
    }
}
