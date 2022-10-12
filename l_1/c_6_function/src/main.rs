fn main() {
    // 1. rust里面函数有定义即可，不在乎放哪里
    // 2. rust里面函数就是表达式
    // 3. 因此和表达式相同，最后一个值不加;就作为返回值，加;隐式返回()
}

fn another_function(x: i32, y: i32) {
    println!("{}", x);
    println!("{}", y);
}

// 4. 永不返回的发散函数，用于导致程序崩溃的函数里面
fn dead_end() -> ! {
    panic!("程序崩溃!")
}

// 无限循环导致永久发散
fn forever() -> ! {
    loop {
        
    };
}