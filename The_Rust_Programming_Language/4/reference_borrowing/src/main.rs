#![allow(unused)]
// 借用和引用，解决所有权传来传去的问题
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
fn main() {
    let s1 = String::from("hello");
    // s1 在栈上保存ptr,len,capacity
    // 传递s1的引用，引用是一个地址，与指针不同，引用确保指向某个特定类型的有效值
    // &s1是一个引用，在栈上保存的是ptr，这个ptr指向栈上的s1而不是指向堆上的空间
    // 将创建一个引用的这种行为称之为借用
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    let mut s = String::from("hello");

    change(&mut s);
    {
        let mut s = String::from("hello");
        // 注意，禁止同时创建两个可变借用，因为这会导致数据竞争
        // 也禁止同时创建可变借用和不可变借用
        // 错误的用法
        // let r1 = &mut s;
        // let r2 = &mut s;
        // 允许的用法
        {
            let r1 = &mut s;
        } // r1 作用域结束了
        let r2 = &mut s;
    }
    {
        // 不可变借用会在第一次使用之后就结束作用域
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s; // 没问题
        println!("{}", r3);
        // 此位置之后r3还能使用
        println!("{}", r3);
    }
}
fn calculate_length(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

// 传递的是可变引用，因此该函数可以改变他借用的值
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Rust编译器会保证不会出现悬垂引用，即引用还保留着但是指向的内存已经释放甚至被分配给其他程序
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回一个借用
// } // 这里s离开作用域并且被释放，因此&s是危险的
// 这里的做法应该是直接把所有权给出去
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}