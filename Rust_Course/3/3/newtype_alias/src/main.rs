use std::ops::Add;

#[allow(unused)]
fn main() {
    // 1. 所谓newtype就是使用元组结构体将已有类型包起来
    {
        struct Meters(u32);
    }
    // 2. newtype的功能:实现特征遵循孤儿规则，可以利用newtype为vector实现Display特征
    {
        struct Wrapper(Vec<String>);

        impl std::fmt::Display for Wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }

    // 3. newtype的功能:更好的可读性以及类型异化
    {
        struct Meters(u32);
        impl std::fmt::Display for Meters {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "目标地点距离{}米", self.0)
            }
        }
        impl Add for Meters {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
            d1 + d2
        }
        let d = calculate_distance(Meters(10), Meters(20));
        println!("{}", d);
    }

    // 4. newtype的功能:隐藏内部的实现细节
    {
        struct Meters(u32);
        let i: u32 = 2;
        assert_eq!(i.pow(2), 4);

        let n = Meters(i);
        // 下面的代码将报错，因为`Meters`类型上没有`pow`方法
        // assert_eq!(n.pow(2), 4);
    }

    // 5. 类型别名
    {
        type Meters = u32;
        // 类型别名并不是一个独立的全新的类型，而是某一个类型的别名，因此编译器依然会把 Meters 当 u32
        // 类型别名仅仅是别名，只是为了让可读性更好，并不是全新的类型，newtype 才是
        // 类型别名除了让类型可读性更好，还能减少模版代码的使用
        // 使用场景主要是如下情况

        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

        fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
            // --snip--
        }

        // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        //     // --snip--
        // }

        // 使用类型别名
        {
            type Thunk = Box<dyn Fn() + Send + 'static>;

            let f: Thunk = Box::new(|| println!("hi"));

            fn takes_long_type(f: Thunk) {
                // --snip--
            }

            // fn returns_long_type() -> Thunk {
            //     // --snip--
            // }
        }
    }

    // 6. Result类型别名
    {
        // 类型别名应用最广的就是简化 Result<T, E> 枚举
        // std::io 库中，它定义了自己的 Error 类型：std::io::Error，那么如果要使用该 Result 就要用这样的语法：std::result::Result<T, std::io::Error>
        // 由于使用 std::io 库时，它的所有错误类型都是 std::io::Error
        // 可以把该错误对用户隐藏起来，只在内部使用即可，因此就可以使用类型别名来简化实现
        type Result<T> = std::result::Result<T, std::io::Error>;
        // 由于它只是别名，因此我们可以用它来调用真实类型的所有方法，甚至包括 ? 符号
    }

    // 7. 永远不返回的类型!
    {
        let i = 2;
        // let v = match i {
        //     0..=3 => i,
        //     _ => println!("不符合规定的值", i)
        // };
        // 编译器报错的原因，如果要使用match赋值，不同分支返回的类型必须要相同
        let i = 2;
        let v = match i {
            0..=3 => i,
            _ => panic!("不合规定的值:{}", i),
        };
        // panic通过了编译，panic的返回类型是!，代表它决不会返回任何值，既然没有任何返回值，那自然不会存在分支类型不匹配的情况
    }
}
