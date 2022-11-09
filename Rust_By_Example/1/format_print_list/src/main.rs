use std::vec;

fn main() {
    // ? 操作符，如果出错，返回相应的错误，没有出错则继续后面的语句
    // try!() 宏效果和?一样
    {
        use std::fmt;
        struct List(Vec<i32>);

        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let vec = &self.0;
                write!(f, "[")?;

                // 使用v迭代vec，并用count记录迭代次数
                for (count, v) in vec.iter().enumerate() {
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", count, v)?;
                }

                write!(f, "]")
            }
        }

        let v = List(vec![1, 2, 3]);
        println!("{v}");
    }
}
