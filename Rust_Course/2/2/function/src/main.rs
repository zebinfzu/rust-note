#[allow(unused)]
fn main() {
    // 1. rust的函数同样是表达式，因此会将{}代码块里最后一个表达式作为返回值
    {
        fn add(i: i32, j: i32) -> i32 {
            i + j
        }
    }
    // 2. rust是强类型语言，因此必须要注明参数列表参数和返回值的类型
    // 3. 没有返回值的函数会隐式返回单元类型的值()
    {
        fn foo_1() {
            println!("no return value");
        }
        fn foo_2() -> () {
            println!("no return value");
        }
    }

    // 4. 发散函数，返回-> !表示这个函数永远不会返回，通常用在会导致panic的函数中
    {
        fn dead_end() -> ! {
            panic!("panic");
        }
        fn forever() -> ! {
            loop {};
        }
        fn foo() -> ! {
            unimplemented!(); // 标记为未不在这里实现的
            todo!(); // 标记为待实现的
        }
    }
}
