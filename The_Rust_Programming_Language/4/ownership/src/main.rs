#![allow(unused)]

// 简单说明ownership，任何堆上的内存都有且同时只有一个变量持有其所有权，当该持有所有权的变量离开了其作用域时，堆上的内存会被drop，除非在此之前发生了move，将堆上内存的所有权转移给了其他变量
fn main() {
    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，
      // s 不再有效，堆上分配的内存被drop
      // rust采用的内存管理规则是，当变量离开了作用域即会被drop，一旦发生了drop就会释放堆上的内存
    {
        // 移动(move)
        let s1 = String::from("ttt");
        // 移动s1栈上的内容到s2上
        let s2 = s1; // 为了避免二次释放，发生了移动，s1之后不再有效
                     // println!("s1 = {}, s2 = {}", s1, s2); 报错，不能继续使用s1
    }
    {
        // 克隆(clone)
        let s1 = String::from("ttt");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    {
        // 拷贝(copy)
        // 只在栈上的数据会发生，因为实现了Copy trait
        // RUST不允许任何包含或者部分包含Drop trait的类型使用Copy trait
        let x = 5;
        let y = x;
    }
    // 函数传参同样会发生移动和复制
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里s不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x

    // 返回值也可以转移所有权
    {
        let s1 = gives_ownership(); // gives_ownership 将返回值
                                    // 转移给 s1

        let s2 = String::from("hello"); // s2 进入作用域

        let s3 = takes_and_gives_back(s2); // s2 被移动到
                                           // takes_and_gives_back 中,
                                           // 它也将返回值移给 s3
    } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
      // 所以什么也不会发生。s1 离开作用域并被丢弃
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动move给
    // 调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域.

    some_string // 返回 some_string
                // 并移出给调用的函数
                //
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    //

    a_string // 返回 a_string 并移出给调用的函数
}
