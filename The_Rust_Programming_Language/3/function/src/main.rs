fn main() {
    another_function(5);
    // rust 是基于表达式的语言
    // statement语句：执行一些操作但不会返回值的指令
    // 函数定义也是语句
    // 不能将语句赋值给另外一个变量因为语句不返回值
    // let y = (let x = 5);

    // expression表达式：
    // 1. 表达式可以组成语句,6是一个表达式
    let y = 6;
    // 2. 函数调用是表达式
    // 3. 宏展开是表达式
    // 4. {}创建的作用域也是一个表达式
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

// rust 的函数参数列表必须声明参数的类型
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// 有返回值的函数
// 可以隐式的返回最后一个表达式的值或者使用return 关键字返回
// 无返回值的函数会返回单元()，通过语句结束函数体
fn five() -> i32 {
    5
}
