use core::panic;
use std::{
    fs::{self, File},
    io::ErrorKind,
};

#[allow(unused)]
fn main() {
    // 1. rust的panic，导致程序崩溃
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    // 2. 代码行为导致的panic
    v[99];
    // 3. panic两种终止方式：栈展示 和 直接终止
    // 在toml文件中,指定[profile.release]字段 panic = 'abort'，直接终止
    // 则release版本直接终止而不是栈展开

    // 4. Result枚举
    // 有时候不希望直接导致程序panic，而是函数返回一个结果，有调用者来决定要不要panic，这时候就会用到Result枚举
    {
        // 标准库定义
        enum Result<T, U> {
            Ok(T),
            Err(U),
        }
    }

    // 5. 对返回错误的处理
    {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }

    // 6.  result值调用函数unwrap(), Ok则返回Ok中的值，Err则panic
    {
        let f = File::open("hello.txt").unwrap();
    }

    // 7. expect和unwrap很像，区别是可以接收一个&str作为panic时候的自定义错误提示
    {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }

    // 8. 错误传播 一般不可能只是A->B，复杂点的项目可能动不动十几层函数调用，因此需要将Err向上不断传播
    {
        use std::io::{self, Read};

        fn read_username_from_file() -> Result<String, io::Error> {
            // 打开文件，f是`Result<文件句柄,io::Error>`
            let f = File::open("hello.txt");

            let mut f = match f {
                // 打开文件成功，将file句柄赋值给f
                Ok(file) => file,
                // 打开文件失败，将错误返回(向上传播)
                Err(e) => return Err(e),
            };
            // 创建动态字符串s
            let mut s = String::new();
            // 从f文件句柄读取数据并写入s中
            match f.read_to_string(&mut s) {
                // 读取成功，返回Ok封装的字符串
                Ok(_) => Ok(s),
                // 将错误向上传播
                Err(e) => Err(e),
            }
        }
    }

    // 9. ? 简化传播的运算符
    {
        use std::io;
        use std::io::Read;

        fn read_username_from_file() -> Result<String, io::Error> {
            // ? 运算符相当于和上个例子的match一样，Ok取出值，Err就让当前函数返回Err
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        // 更短的操作，从文件读取到字符串中太常见，所以标准库提供了函数fs::read_to_string
        fn read_username_from_file_0() -> Result<String, io::Error> {
            // read_to_string是定义在std::io中的方法，因此需要在上面进行引用
            fs::read_to_string("hello.txt")
        }
    }

    // 10. ? 操作符不仅可以用于Result的错误传播，也可以用于Option值的传播
    {
        fn first(arr: &[i32]) -> Option<&i32> {
            let v = arr.get(0)?; // 是None的话返回None
            Some(v)
        }
        // 当然上面这个函数根本不必要使用?
        fn first_0(arr: &[i32]) -> Option<&i32> {
            arr.get(0)
        }
    }

    // 11. 注意?操作符需要一个变量来存放正确的值，所以不要写如下的代码，会编译报错
    {
        // fn first(arr: &[i32]) -> Option<&i32> {
        //     arr.get(0)?
        // }
    }

    // 12. 带有返回值的main函数
    {
        // 之前提到了?在没有值的情况下或者错误的情况下会返回
        // 那么在主函数当中要使用?就必须使用带有返回值的main函数
        // fn main() -> Result<(), Box<dyn Error>> {}
    }
}
