// 标准库提供的枚举Result
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // 1. 以函数的返回值Result的具体类型来判别是不是需要panic
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            {
                // 以文件读写为例
                let f = File::open("hello.txt");

                let mut f = match f {
                    Ok(file) => file,
                    Err(e) => return Err(e),
                };

                let mut s = String::new();

                match f.read_to_string(&mut s) {
                    Ok(_) => Ok(s),
                    Err(e) => Err(e),
                }
            }
        }
        match read_username_from_file() {
            Ok(_) => (),
            Err(_) => (), // 可以匹配Error来panic，并对不同的Err做处理
        }
    }
    // 2. 传播错误的运算符?
    {
        #[allow(dead_code)]
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            // 使用?与1中的match相同，表达式返回Ok中的值并且会继续执行，如果返回Err函数就会直接返回Err，后续代码不会继续执行
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        // 使用?运算符可以消除大量多余的代码
        #[allow(dead_code)]
        fn read_username_from_file_() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
}
