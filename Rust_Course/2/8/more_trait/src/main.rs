use std::ops::Add;

#[allow(unused)]
fn main() {
    // 1. 关联类型
    // 指的是在特征定义语句块中，申明一个自定义类型，这样就可以在特征的方法签名中使用该类型
    {
        // 标准库中迭代器特征Iterator，有一个Item关联类型，用于体改遍历的值的类型
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        // 为什么不使用泛型
        {
            pub trait Iterator<Item> {
                fn next(&mut self) -> Option<Item>;
            }
        }
        // 这是为了代码可读性，当所有地方都需要协商Iterator<Item>的时候可读性会很差，类型定义复杂的例子
        // 只写Address可比泛型写AsRef<[u8]> + Clone + fmt::Debug + Eq来的可读性强
        {
            pub trait CacheableItem: Clone + Default + std::fmt::Debug {
                type Address: AsRef<[u8]> + Clone + std::fmt::Debug + Eq;
                fn is_null(&self) -> bool;
            }
        }
        // 再比如
        {
            trait Container<A, B> {
                fn container(&self, a: A, b: B) -> bool;
            }
            fn difference<A, B, C>(container: &C) -> i32
            where
                C: Container<A, B>,
            {
                1
            }
        }
        // 使用关联类型则可读性强很多
        {
            trait Container {
                type A;
                type B;
                fn container(&self, a: &Self::A, b: &Self::B) -> bool;
            }
            fn difference<C: Container>(container: &C) -> i32 {
                1
            }
        }
    }
    // 2. 默认泛型类型参数
    {
        // 例如Add特征
        // trait Add<RHS = Self> {
        //     type Output;
        //     fn add(self, rhs: RHS) -> Self::output;
        // }

        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, rhs: Self) -> Point {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
        // 实现Add特征提供了运算符重载，对于Point类型可以使用+运算符了，操作符左右必须都是Point
        println!("{:?}", Point { x: 1, y: 0 } + Point { x: 1, y: 2 });

        // 重载两个不同类型的相加
        #[derive(Debug)]
        struct Millimeters(u32);

        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, rhs: Meters) -> Self::Output {
                Millimeters(self.0 + (rhs.0 * 1000))
            }
        }

        println!("{:?}", Millimeters(10) + Meters(10));
    }
    // 3. 调用同名方法
    {
        // 不同特征拥有同名的方法

        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        // 优先调用类型上面的方法
        let person = Human;
        person.fly();

        // 通过特征显式调用特征的方法
        Pilot::fly(&person);

        Wizard::fly(&person);

        // 但是如果方法没有self参数，即方法是关联函数的时候

        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", Dog::baby_name());
        // println!("A baby dog is called a {}", Animal::baby_name()); // 这样是不行的，因为Animal的baby_name方法不是一个具体的限定，编译器不知道要调哪一个
        
        // 使用完全限定语法 通过 as 关键字提供准确的类型注解
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
}
