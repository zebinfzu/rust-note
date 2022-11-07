/*
enum Option<T> {
    Some(T),
    None
}
*/
#![allow(unused)]
fn main() {
    // 使用Option类型
    // match匹配的时候一定要处理完所有情况
    {
        let some_number = Some(5);
        let some_string = Some(String::from("a string"));
        let absent_number: Option<i32> = None;

        let x = 5;
        let y = Some(5);
        let sum = x + match y {
            Some(i) => i,
            None => 0,
        };
        println!("{sum}");
    }
    // 函数里面使用Option类型
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(x) => Some(x + 1),
            }
        }
        let result = plus_one(Some(7));
    }

    // if let
    {
        let f = Some(7);
        if let Some(value) = f {
            println!("{value}")
        } else {
            println!("nothing")
        }
    }
}
