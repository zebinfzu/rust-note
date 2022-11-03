#![allow(unused)]
use std::thread;
use std::time::Duration;

fn main() {
    // 案例模拟
    {
        // 一个模拟计算数值的函数，假定耗时在2s左右
        fn simulated_expensive_calculation(intensity: u32) -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

        // intensity 表示用户喜欢高强度还是低强度，以及一个随机数，产生随机的变化
        fn generate_workout(intensity: u32, random_number: u32) {
            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    simulated_expensive_calculation(intensity)
                );
                println!(
                    "Next, do {} situps!",
                    simulated_expensive_calculation(intensity)
                );
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        simulated_expensive_calculation(intensity)
                    );
                }
            }
        }
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }
    // 重构代码，希望simulated_expensive_calculation只被调用一次，这样当该函数被修改的时候，其他要修改的地方就只有一处
    {
        fn simulated_expensive_calculation(intensity: u32) -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }
        fn generate_workout(intensity: u32, random_number: u32) {
            let expensive_result = simulated_expensive_calculation(intensity);
            // 这样解决了在第一个代码块中需要调用两次的问题，但是这样不管哪个分支都需要调度一次，即使是原来不需要函数调度的分支
            if intensity < 25 {
                println!("Today, do {} pushups!", expensive_result);
                println!("Next, do {} situps!", expensive_result);
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!("Today, run for {} minutes!", expensive_result);
                }
            }
        }
    }
    // 闭包重构
    {
        fn generate_workout(intensity: u32, random_number: u32) {
            // 将原来的函数体存储到闭包里面
            // || 中间是闭包的参数，后面{}是存放闭包的大括号，如果这个闭包只有一行则可以无视掉{}
            // 现在如何执行复杂计算只被定义了一次，并只会在需要结果的时候执行该代码
            // 但是又有了最开始的问题，第一个if代码块中调用了两次
            let expensive_closure = |num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            };

            if intensity < 25 {
                println!("Today, do {} pushups!", expensive_closure(intensity));
                println!("Next, do {} situps!", expensive_closure(intensity));
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!("Today, run for {} minutes!", expensive_closure(intensity));
                }
            }
        }
    }
    // 闭包类型注解
    {
        fn add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| x + 1;
        add_one_v3(1);
    }
    // 带有泛型和Fn trait的闭包
    {
        // 有多个计算慢闭包的地方，可以创建一个存放闭包和调用闭包结果的结构体
        // memoization 或 lazy evaluation （惰性求值）
        // 需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型
        // Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
        // Cache有一个泛型参数T，该泛型T特征绑定了使用Fn的闭包
        struct Cache<T>
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cache<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cache<T> {
                Cache {
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

        fn generate_workout(intensity: u32, random_number: u32) {
            let mut expensive_result = Cache::new(|num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            });

            if intensity < 25 {
                println!("Today, do {} pushups!", expensive_result.value(intensity));
                println!("Next, do {} situps!", expensive_result.value(intensity));
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_result.value(intensity)
                    );
                }
            }
        }
        // Cache存在的问题
        // 第一个问题是 Cache 实例假设对于 value 方法的任何 arg 参数值总是会返回相同的值。也就是说，这个 Cache 的测试会失败
        fn call_with_different_values() {
            let mut c = Cache::new(|a| a);
            let v1 = c.value(1);
            let v2 = c.value(2);
            assert_eq!(v2, 2);
        }
    }
    // 闭包会捕获当前环境
    {
        let x = 4;
        // x不在闭包的参数列表里面，但是却可以使用，函数做不到
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));
        // 闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用
        // FnOnce特征 闭包周围的作用域被称为其 环境，environment 闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
        // FnMut 获取可变的借用值所以可以改变其环境
        // Fn 从其环境获取不可变的借用值
    }
}
