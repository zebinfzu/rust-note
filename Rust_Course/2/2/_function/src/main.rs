#![allow(unused)]
fn main() {
    // 函数 fn 签名<泛型参数>(参数: 类型) -> 返回值类型 {}
    {
        fn add(i: i32, j: i32) -> i32 {
            i + j
        }
    }
    // 函数的没有返回值就是返回单元unit，此时以;的语句结束就是没有返回值
    {
        fn foo() {
            let i = 1;
        }
    }
    // 函数发散，即永远不会返回，有两种情况会发散，1是panic，2是无限不会跳出的循环
    {
        fn foo() -> ! {
            panic!("崩溃");
        }
        fn baz() -> ! {
            loop {

            };
        }
    }
}
