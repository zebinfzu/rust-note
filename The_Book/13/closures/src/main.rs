use std::thread;
use std::time::Duration;

#[allow(unused)]
fn main() {
    // 1. 函数案例
    {
        // 通过计算获取一个值，但模拟计算过程非常耗时
        fn fn_input(input: u32) -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2)); // 线程睡2s
            let output = input;
            output
        }

        // 通过计算的固定值和随机数来获取范围
        fn get_result(input: u32, random: u32) {
            // 同一个逻辑内，fn_input的入参完全相同，却要在不同的地方被调用
            if input < 25 {
                println!("small input:{}", fn_input(input));
            } else {
                if random == 3 {
                    println!("random -- 3:{}", fn_input(input));
                } else {
                    println!("random -- !3:{}", fn_input(input));
                }
            }
        }

        let (input, random) = (10, 3);
        get_result(input, random);
    }
    // 2. 使用函数重构此程序
    {
        fn fn_input(input: u32) -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2)); // 线程睡2s
            let output = input;
            output
        }

        fn get_result(input: u32, random: u32) {
            // 提升变量，减少编码fn_input的调用次数
            let calculate = fn_input(input);
            if input < 25 {
                println!("small input:{}", calculate);
            } else {
                if random == 3 {
                    println!("random -- 3:{}", calculate);
                } else {
                    println!("random -- !3:{}", calculate);
                }
            }
        }
    }
    // 3. 重构，使用闭包来存储结果
    {
        fn get_result(input: u32, random: u32) {
            // 定义闭包
            let calculate = |input: u32| -> u32 {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2)); // 线程睡2s
                let output = input;
                output
            };
            // 此时任然调用了多次
            if input < 25 {
                println!("small input:{}", calculate(input));
            } else {
                if random == 3 {
                    println!("random -- 3:{}", calculate(input));
                } else {
                    println!("random -- !3:{}", calculate(input));
                }
            }
        }
    }

    // 闭包的定义形式
    {
        fn add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| x + 1;
        let add_one_v4 = |x| x + 1;
        add_one_v3(4);
        add_one_v4(5);
    }

    // 对于上面闭包的问题：可以将结果保存进变量以供复用，这样就可以使用变量而不是再次调用闭包。但是这样就会有很多重复的保存结果变量的地方
    // 解决方案的一种: 创建一个存放闭包和调用闭包结果的结构体
    // 只会在需要结果时执行闭包，并会缓存结果值
    // 所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
    {
        // 只有第一次调用的时候会计算，之后使用的是缓存值
        struct Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        fn get_result(input: u32, random: u32) {
            // 使用有缓存的闭包
            let mut calculate = Cacher::new(|num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2)); // 线程睡2s
                num
            });
            // 此时任然调用了多次
            if input < 25 {
                println!("small input:{}", calculate.value(input));
            } else {
                if random == 3 {
                    println!("random -- 3:{}", calculate.value(input));
                } else {
                    println!("random -- !3:{}", calculate.value(input));
                }
            }
        }
    }

    // 上面的cacher版本对于多次输入不同的arg会返回相同的值，设计一个可以缓存并且输入不同arg能返回不同值的cacher
    {
        use std::collections::HashMap;

        struct Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            value: HashMap<u32, u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: HashMap::new(),
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value.get(&arg) {
                    Some(v) => *v,
                    None => {
                        println!("call closure");
                        thread::sleep(Duration::new(2, 0));
                        let v = (self.calculation)(arg);
                        self.value.insert(arg, v);
                        v
                    }
                }
            }
        }

        let mut call = Cacher::new(|num| num + 5);
        println!("{}", call.value(10));
        println!("{}", call.value(10));
        println!("{}", call.value(5));
        println!("{}", call.value(5));
        println!("{}", call.value(7));
        println!("{}", call.value(7));
    }

    // 通过闭包捕获上下文环境的值
    {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));
    }

    // 闭包捕获环境变量的三种方式
    // 1. FnOnce 消耗环境变量，直接拿走所有权
    // 2. FnMut 获取可变借用
    // 3. Fn 获取不可变借用

    {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x; // 定义的时候x就move给闭包了，而非调用的时候才发生move

        // println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }

    // FnMut
    {
        let mut x = 1;
        let add_x = || x += 1;
    }
}
