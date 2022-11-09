fn main() {
    // 想要用std::fmt的格式化打印，至少要实现一个可打印的traits，仅有一些类型提供了自动实现
    // fmt::Debug这个trait都可以使用derive自动推导，而display需要手动实现
    {
        #[derive(Debug)]
        struct DebugPrintable(i32);
        // 使用{:?}作为标准输出的占位符
        // 通过{:#?}美化打印

        #[derive(Debug)]
        struct Deep(DebugPrintable);

        println!("{}", 1);
        println!("{:?}", 1);

        println!("{:?}", Deep(DebugPrintable(1)));
        println!("{:#?}", Deep(DebugPrintable(2)));
    }
}
