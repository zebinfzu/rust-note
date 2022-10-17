#![allow(unused)]
fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

// 泛型函数：编译不过
// fn add<T>(a: T, b: T) -> T {
//     a + b
// }

// 一样编译不过
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 上面编译不过的原因是函数传参泛型T，但不是所有类型都实现了+,>这些操作符

// 因此需要对泛型进行限制，限制T实现了+
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 结构体里面使用泛型
fn foo1() {
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// 枚举里面使用泛型
fn foo2() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

// 方法里面使用泛型
fn foo3() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // mixup上面的是函数泛型和<T, U>并不冲突
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}

// 单独给具体特定类型实现方法
fn foo4() {
    struct Point<T> {
        x: T,
        y: T,
    }
    // 单独给f32类型的Point实现方法，其他类型的Point实例没有该方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
}

// 1.51版本重要特性，const泛型 -> 针对值的泛型
// const泛型是值的泛型 const N: usize 表示值类型usize
// 在没有const泛型之前rust很难用于复杂的矩阵运算
fn foo5() {
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

fn main() {
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));
}
