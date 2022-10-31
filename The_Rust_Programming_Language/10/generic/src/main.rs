#![allow(unused)]
fn main() {
    {
        // 函数中定义泛型
        // 函数接收的泛型要受到函数实现的约束
        // 比如这个函数里面要做泛型的 > 比较，因此要约束输入的泛型T实现了>
        // fn largest<T>(list: &[T]) -> T {
        //     let mut largest = list[0];
        //     for &item in list {
        //         if item > largest {
        //             largest = item;
        //         }
        //     }
        //     largest
        // }
    }
    {
        // 结构体泛型
        struct Point<T> {
            x: T,
            y: T,
        }
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        // let wont_work = Point { x: 5, y: 4.0 }; 错误，一个泛型参数只能指代一种类型
        // 泛型参数列表，一个参数只代表一种类型
        struct PPoint<T, U> {
            x: T,
            y: U,
        }
        let wont_work = PPoint { x: 5, y: 4.0 };
    }
    {
        // 枚举中定义泛型，类似Option和Result
        enum Option<T> {
            Some(T),
            None,
        }
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }
    {
        // 方法中的泛型
        struct Point<T> {
            x: T,
            y: T,
        }
        // 注意在impl之后必须加上<T>
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        // 或者不在impl之后加<T>，而是单独限制某种具体类型实现方法
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
    }
    {
        // 方法泛型的参数列表可以和结构体的参数列表不同
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
