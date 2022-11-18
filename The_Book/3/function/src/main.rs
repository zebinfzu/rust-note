// the most important functions in language: the main function, which is the entry point of many programs.
fn main() {
    // 1. 定义函数 fn name() -> return value type {}
    {
        fn foo() -> i32 {
            3
        }
        println!("{}", foo());
    }
    // 2. parameters 定义时参数列表称为形参，调用时传入的称为实参
    {
        fn another_function(x: i32) {
            println!("The value of x is: {}", x);
        }
        another_function(5);
    }
    // 3. statement
    {
        // 执行不返回值的指令
        let _y = 6;
        // let x = (let y = 6); // 语句不能返回值，因此编译报错
    }
    // 4. expression
    {
        // {} 是一个表达式
        let y = { 3 };
        println!("{}", y);
    }
}
