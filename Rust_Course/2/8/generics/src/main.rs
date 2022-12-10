#[allow(unused)]
fn main() {
    // 1. rust的泛型参数
    {
        // 函数泛型参数写在函数名后面的<>中
        fn foo<T>(a: T) {}
        // 结构体的泛型参数写在类型名后面的<>中
        // 并且拒绝声明但未使用的泛型参数，这样会导致报错
        struct Point<T> {
            x: T,
            y: T,
        }
        let p = Point { x: 1, y: 2 };
        let p = Point { x: 1.1, y: 2.0 };

        // 枚举泛型
        enum Option<T> {
            Some(T),
            None,
        }

        // 方法泛型，需要在impl块后面提前声明泛型参数，提前声明的泛型参数才可以在方法里面使用
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &&self.x
            }
        }

        // 在方法里面还可以定义额外的方法泛型参数
        struct PPoint<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> PPoint<T, U> {
            fn mixup<V, W>(self, other: PPoint<V, W>) -> PPoint<T, W> {
                PPoint {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        // 为具体泛型类型实现方法
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
    }
    // 2. const泛型
    {
        // 只能接收[i32;3]类型的参数
        fn display_array_0(arr: [i32; 3]) {
            println!("{:?}", arr);
        }
        // 通过切片来接收任意长度的数组
        fn display_array_1(arr: &[i32]) {
            println!("{:?}", arr);
        }
        // 通过泛型参数来接收任意类型任意长度的数组
        // 并且对泛型参数做限制，必须实现了Debug特征
        fn display_array_2<T: std::fmt::Debug>(arr: &[T]) {
            println!("{:?}", arr);
        }
        // 对于一些特殊场景，就是要传数组而不是传数组的切片
        // 利用const泛型来实现指定值
        fn display_array_3<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
            println!("{:?}", arr);
        }
    }
    // 3. 泛型函数调用的方式
    {
        struct A; // 类单元结构体，类名也是其唯一值
        let t = A;
        struct S(A); // 类元组结构体
        struct SGen<T>(T); // 包含泛型的类元组结构体
        fn reg_fn(_s: S) {}
        fn gen_spec_t(_s: SGen<A>) {}
        fn gen_spec_i32(_s: SGen<i32>) {}
        fn generic<T>(_s: SGen<T>) {}

        // 使用非泛型函数
        reg_fn(S(A)); // 具体的类型 函数调用的时候传的必须是值，这里调用S的构造函数，传的A是A类型的值
        gen_spec_t(SGen(A)); // 隐式地指定类型参数  `A`.
        gen_spec_i32(SGen(6)); // 隐式地指定类型参数`i32`.

        // 显式地指定类型参数 `char`
        generic::<char>(SGen('c'));

        // 隐式地指定类型参数 `char`.
        generic(SGen('b'));
    }
    // 4. 对方法的类型限制
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        // 希望对实现了T是实现了Debug特征的Point实现特定方法
        // 在impl块提前声明中限制而不是修改Point的泛型参数列表
        impl<T: std::fmt::Debug> Point<T> {
            fn print(&self) {
                println!("{:?}, {:?}", self.x, self.y);
            }
        }
    }
    // 5. const泛型只能支持以下形式的实参
    {
        fn foo<const N: usize>() {}

        fn bar<T, const M: usize>() {
            // 1. 一个单独的const泛型参数
            foo::<M>(); // ok

            // 2. 一个字面量值
            foo::<2022>(); // ok

            // 3. 一个具体的const表达式(里面不能包含任何泛型参数)
            foo::<{ 20 * 100 + 20 * 10 + 1 }>();

            // foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M

            // foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T

            let _: [u8; M]; // ok: 符合第一种

            // let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
        }
    }
}
