#[allow(unused)]
fn main() {
    // 1. rust编译期不能确定大小的类型称为dynamically sized types，运行时才能确定大小，如Vector、String、HashMap
    // 上述的这些集合虽然底层数据可动态变化，感觉像是动态大小的类型。但是实际上，这些底层数据只是保存在堆上，在栈中还存有一个引用类型
    // 栈上的引用类型是固定大小的
    // 正因为编译器无法在编译期获知类型大小，若你试图在代码中直接使用 DST 类型，将无法通过编译
    {
        // 编译报错，试图创建动态类型的数组
        // fn my_function(n: usize) {
        //     let arr = [123;n];
        // }
    }
    // 切片也是一个典型的 DST 类型
    // str也是DST类型
    // 还有特征对象

    // 2. 结论，无法直接使用DST，只能间接地使用DST

    // 3. 泛型参数保证是固定大小是因为编译器自动添加了Sized特征约束
    {
        fn generic<T>(t: T) {
            // --snip--
        }

        // 编译器加上了约束
        fn generic_<T: Sized>(t: T) {}
    }

    // 4. 通过Box来间接使用DST
    {
        // let s1: Box<str> = Box::new("hello DST!" as str); 主动as的方式不行
        // 让编译器去推导
        let s1: Box<str> = "Hello there!".into();
    }
}
