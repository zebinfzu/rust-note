#![allow(unused)]
use std::fs;
use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};
fn main() {
    {
        // 使用match表达式处理Result成员
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }
    {
        // 匹配不同的错误
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }
    {
        // 使用闭包的相同实现，闭包具体参考13章
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }
    {
        // match相对来说用在错误处理有点冗长
        // Result<T, E>定义了很多方法来辅助处理各种情况
        // unwrap Err的时候自动panic
        let f = File::open("hello.txt").unwrap();
        // expect 相比于unwrap可以传入一个字符串作为错误信息
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    {
        // 传播错误
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e), // 出错的时候没有panic而是作为函数结束，返回了Err值
                                         // 因此出错，函数的调用者会获得一个Err值
            };
            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
    }
    {
        // 使用?简写传播错误
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        // 使用链式调用继续简化代码
        fn _read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
        // 由于这种操作太过常用
        fn _read_username_from_file_() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
        // ?运算符只能用于和返回值与?运算符兼容的函数
        // 函数返回值是Result或者Option，可以使用?
    }
}
