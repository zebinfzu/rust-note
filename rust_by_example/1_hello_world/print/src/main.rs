fn main() {
    // 1. 格式化输出的几个相关的宏
    {
        // format!: 这个宏将格式化后的内容返回一个String
        // print!: 这个宏将格式化后的内容输出到标准输出
        // println!: 相比print!在末尾多加一个换行符
        // eprint!: 相比print!输出到标准错误输出
        // eprintln!
    }
    // 2. 以上的宏对文本解析的方式相同
    {
        // 第一个参数接收一个字符串字面量
        // 中间插入的{}会被后续的变量参数取代，按照变量的Display特征的具体实现来取代{}
        println!("{} days", 31);
        // {:?}按照Debug特征的实现来取代变量
        println!("name: {:?}", "John");
        // {}可以填入数字指明使用第几个参数
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        // 可以使用命名参数。
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );

        // 可以在 `:` 后面指定特殊的格式。
        println!("{} of {:b} people know binary, the other half don't", 1, 2);

        // 可以按指定宽度来右对齐文本。
        // 下面语句输出 "     1"，5 个空格后面连着 1。
        println!("{number:>width$}", number = 1, width = 6);

        // 
    }
    // 3. 自定义数据类型要使用格式化输出必须实现Debug特征或者Display特征
    {
        // 通过derive自动实现Debug
        #[derive(Debug)]
        struct DebugPrintable(i32);

        #[derive(Debug)]
        struct Deep(DebugPrintable);

        // 实现了Debug特征的类型可以使用{:?}来输出到格式化输出
        println!("Now {:?} will print!", DebugPrintable(3));
        println!("Now {:?} will print!", Deep(DebugPrintable(7)));

        // Display特征不能通过derive自动实现
        struct Structure(i32);

        impl std::fmt::Display for Structure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        
    }
}
