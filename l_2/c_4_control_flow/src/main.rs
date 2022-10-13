fn main() {
    // rust 里面的if else 接收一个表达式作为condition，{}作为表达式会返回值
    let n = 6;
    let n = if n % 2 == 0 { 0 } else { 1 };
    println!("{}", n);
    // for
    // 1. for 元素 in 集合
    for i in 1..=5 {
        println!("{}", i);
    }
    // 2. 通常会转换成引用，否则会导致所有权的move，for循环之后就不能使用这个集合了
    // for item in collection 所有权转移
    // for item in &collection 不可变借用
    // for item in &mut collection 可变借用
    // 3. 想在循环中使用元素的索引
    let a = [1, 2, 4, 5, 1];
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
    // 不想单独声明一个变量来控制这个流程
    for _ in 0..10 {
        println!("{}", 0);
    }
    // 具有continue和break和其他语言一样

    // while loop，没有do while
    let mut n = 0;
    // loop就是单一无限的循环，要跳出需要使用break
    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n += 1;
    }

    println!("我出来了！");
}
