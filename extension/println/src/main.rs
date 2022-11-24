fn main() {
    // print! 格式化输出到标准输出，末尾不包含/n
    // eprint!          错误输出
    // println! 格式化输出到标准化输出，尾部包含/n
    // eprintln!
    // format! 格式化输出到字符串
    // {} 是格式化占位符，适用于实现了std::fmt::Display特征的类型
    // {:?}, {:#?} 同样是格式化占位符，适用于实现了std::fmt::Debug特征的类型，通常用于调试数据

    // 1. 参数位置，除了按照依次顺序使用值去替换占位符之外，还能让指定位置的参数去替换某个占位符
    {
        println!("{}{}", 1, 2); // =>"12"
        println!("{1}{0}", 1, 2); // =>"21"
                                  // => Alice, this is Bob. Bob, this is Alice
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
        println!("{1}{}{0}{}", 1, 2); // => 2112
    }

    // 2. 具名参数，可以给参数指定名称，并在{}占位符中使用
    {
        println!("{argument}", argument = "test"); // => "test"
        println!("{name} {}", 1, name = 2); // => "2 1"
        println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
    }

    // 3. 格式化输出
    {
        // 指定小数点之后的位数
        let pi = 3.1415926;
        // Display
        println!("{:.2}", pi);
        // Debug
        println!("{:.2?}", pi);

        // 宽度，指定输出的总长度，字符串会默认用空格填充，并左对齐
        // {:5} 指定占位长度为5，不够用空格填补
        println!("Hello {:5}!", "x");
        // {:1$} 指定输出总长度是参数1
        println!("Hello {:1$}!", "x", 5);
        // {1:0$} 指定占位符使用参数1，长度使用参数0
        println!("Hello {1:0$}!", 5, "x");
        // 使用有名称的参数作为宽度
        println!("Hello {:width$}!", "x", width = 5);

        // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
        println!("Hello {:1$}!{}", "x", 5);

        // 数字格式化一样默认空格填充，但是右对齐
        // 宽度是5 => Hello     5!
        println!("Hello {:5}!", 5);
        // 显式的输出正号 => Hello +5!
        println!("Hello {:+}!", 5);
        // 宽度5，使用0进行填充 => Hello 00005!
        println!("Hello {:05}!", 5);
        // 负号也要占用一位宽度 => Hello -0005!
        println!("Hello {:05}!", -5);

        // 以下全部都会补齐5个字符的长度
        // 左对齐 => Hello x    !
        println!("Hello {:<5}!", "x");
        // 右对齐 => Hello     x!
        println!("Hello {:>5}!", "x");
        // 居中对齐 => Hello   x  !
        println!("Hello {:^5}!", "x");

        // 对齐并使用指定符号填充 => Hello x&&&&!
        // 指定符号填充的前提条件是必须有对齐字符
        println!("Hello {:&<5}!", "x");
    }

    // 4. 精度控制
    {
        let v = 3.1415926;
        // 保留小数点后两位 => 3.14
        println!("{:.2}", v);
        // 带符号保留小数点后两位 => +3.14
        println!("{:+.2}", v);
        // 不带小数 => 3
        println!("{:.0}", v);
        // 通过参数来设定精度 => 3.1416，相当于{:.4}
        println!("{:.1$}", v, 4);

        let s = "hi我是Sunface孙飞";
        // 保留字符串前三个字符 => hi我
        println!("{:.3}", s);
        // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
        println!("Hello {:.*}!", 3, "abcdefg");
    }

    // 5. 指数输出
    {
        println!("{:2e}", 1000000000); // => 1e9
        println!("{:2E}", 1000000000); // => 1E9
    }

    // 6. 内存指针地址
    {
        let v = vec![1, 2, 3];
        println!("{:p}", v.as_ptr()); // 输出栈上的指针值
    }

    // 7. 进制，{:o}, {:b}, {:x}
    {
        println!("0o{:o}", 1234);
    }
}
