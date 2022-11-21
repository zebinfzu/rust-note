fn main() {
    // 1. 从命令行读取
    {
        use std::io::stdin;

        // 1. 申明一个可变字符串作为命令行读入的缓冲区
        let mut buf = String::new();

        // 2. 命令行的读入是在缓冲区的末尾追加的
        // 按行读入，因此读到回车结束
        stdin().read_line(&mut buf)
            .expect("Failed to read line");

        println!("{}", buf);
    }
}
