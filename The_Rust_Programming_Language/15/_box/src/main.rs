fn main() {
    // 最简单的智能指针，Box<T>
    //  box 允许你将一个值放在堆上而不是栈上, 留在栈上的则是指向堆数据的指针
    {
        let b = Box::new(5);
        println!("{}", b);
    }
    // Rust 需要在编译时知道类型占用多少空间
    // 一种无法在编译时知道大小的类型是 递归类型
    // 其值的一部分可以是相同类型的另一个值
    // 这种值的嵌套理论上可以无限的进行下去
    {
        // 无法编译，因为枚举不能创建递归类型
        // enum List {
        //     Cons(i32, List), // 包含一个相同的List, 这样在编译器检查需要多大的容量存储一个List的时候就会无限递归检查下去
        //     Nil
        // }
        // 使用 Box<T> 给递归类型一个已知的大小
        enum List {
            Cons(i32, Box<List>), // Cons 是一个元组包含一个 i32 和一个 Box<List>， Box有固定大小和usize相同
            Nil
        }
        use List::{Cons, Nil};
        let _list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    }
}
