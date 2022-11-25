// 1. 会报错的泛型函数，因为函数中使用了T > 并不是所有类型都实现了 >
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

use std::fmt::Debug;

// 2. 正确的在一个切片中找最大元素的泛型函数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 3. 泛型函数签名 fn function_name<generic_data_type: must complete trait> (arg) -> return Type
    {
        #[allow(dead_code)]
        fn largest<T>(n: T) -> T {
            n
        }
    }

    // 4. 结构体和枚举，以及方法中的泛型
    {
        #![allow(unused)]

        // 泛型T
        #[derive(Debug)]
        struct Test<T> {
            x: T,
            y: T,
        }

        enum TTest<T, U> {
            Ok(T), // 一个叫Ok的枚举成员绑定到泛型变量T
            Err(U),
        }

        // 实现方法，泛型T满足具有Clone方法的时候，Test对象可以调用Clone方法
        // impl<> 所给定的参数必须泛型范围<= Test<T>
        // impl<> 中声明泛型是为了让编译器知道， 后面类型<>中的是泛型而非具体类型
        impl<T: Clone> Test<T> {
            fn clone(&self) -> Self {
                Test {
                    x: self.x.clone(),
                    y: self.y.clone(),
                }
            }
        }

        // 或者为具体类型实现方法
        impl Test<i32> {
            fn convert(&self) -> Self {
                Test {
                    x: self.y,
                    y: self.x,
                }
            }
        }
    }
}
