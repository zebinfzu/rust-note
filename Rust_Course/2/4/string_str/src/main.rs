#[allow(unused)]
fn main() {
    // 1. 切片
    // Rust的切片允许引用集合中部分元素序列
    {
        let s = String::from("hello World");
        // 切片语法 [开始索引..结束索引] 返回的切片[)
        let hello = &s[0..5];
        let world = &s[6..11];
        // s: ptr -> 堆上的字符串 len: 11 capacity: 11
        // world: ptr -> 堆上的切片起始位置 len: 5
    }
    // 2. 序列语法
    {
        let s = String::from("hello");
        let len = s.len();
        let slice = &s[4..]; // 切片包含到最后一个字节
        let slice = &s[..4]; // 切片从第一个字节开始
        let slice = &s[..]; // 引用整个字符串的切片
        let slice = &s[0..=2]; // 左闭右闭切片
    }
    // 3. 字符串字面量是切片
    {
        let s = "hello world";
    }
    // 4. Rust中的字符char是Unicode类型，因此每一个字符占据4byte
    // 5. 但Rust的String是UTF-8编码，也就是一个字符串中的字符占据(1-4)byte
    // 6. String和&str的转换
    {
        // 函数接收&String可以自动转换为&str，这是因为String类型实现了Deref特征
        let s = String::from("hello,world!");
        say_hello(&s);
        say_hello(&s[..]);
        say_hello(s.as_str());
        fn say_hello(s: &str) {
            println!("{}", s);
        }
    }
    // 7. 字符串索引
    {
        // 前面说过，String中的字符长度不固定，因此不要用下标索引
        let hello = String::from("中国人");
        println!("{:?}", &hello.as_bytes());
        // 同理使用切片的时候也可能切到不完整的字符而导致panic
        // let s = &hello[0..2]; // 会导致panic
    }
    // 8. String常用方法
    {
        // push
        let mut s = String::from("Hello ");
        s.push('r');

        // insert(index, char)，越界会发生错误
        let mut s = String::from("Hello rust!");
        s.insert(5, ',');

        // replace 用于替换字符串，会替换所有匹配到的字符串，这个方法对String和&str都可以使用
        let string_replace = String::from("I like rust. Learning rust is my favorite!");
        // 返回的是新的String，而不是修改了原串
        let new_string_replace = string_replace.replace("rust", "RUST");

        // replacen 和replace的区别在于多了第三个参数，指定要替换的个数
        let string_replace = "I like rust. Learning rust is my favorite!";
        let new_string_replacen = string_replace.replacen("rust", "RUST", 1);

        // replace_range(range, content) 直接操作原串，指定替换范围和内容
        let mut string_replace_range = String::from("I like rust!");
        string_replace_range.replace_range(7..8, "R");

        // 删除 pop remove truncate clear 只适用于String
        // pop 删除并返回最后一个字符(Option)，直接操作原串
        let mut string_pop = String::from("rust pop 中文!");
        let p1 = string_pop.pop();
        let p2 = string_pop.pop();

        // remove 删除并返回字符串中指定位置的字符，直接操作原串
        // 这个方法按字节处理字符串，所以同样传入的字节有问题会导致错误
        let mut string_remove = String::from("测试remove方法");
        println!(
            "string_remove 占 {} 个字节",
            std::mem::size_of_val(string_remove.as_str())
        );
        // 删除第一个汉字
        string_remove.remove(0);
        // 下面代码会发生错误
        // string_remove.remove(1);
        // 直接删除第二个汉字
        // string_remove.remove(3);

        // truncate 删除字符串中从指定位置开始到结尾的全部字符，同样是按字节操作，可能会导致错误
        // clear 直接清空原串

        // 链接
        // + += 要求右边的操作数是&str
        let string_append = String::from("hello ");
        let string_rust = String::from("rust");
        // &string_rust会自动解引用为&str
        let result = string_append + &string_rust;
        let mut result = result + "!";
        result += "!!!";
    }
    // 9. 操作UTF-8字符串的方式
    {
        // 转换成char，当作Unicode字符处理
        for c in "中国人".chars() {
            println!("{}", c);
        }
        // 当作字节处理
        for b in "中国人".bytes() {
            println!("{}", b);
        }
    }
    // 10. 字符串转义
    {
        // 通过 \ + 字符的16进制转义一个字符串
        let byte_escape = "I'm writing \x52\x75\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

        // \u 可以输出一个 unicode 字符
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!(
            "Unicode character {} (U+211D) is called {}",
            unicode_codepoint, character_name
        );

        // 换行了也会保持之前的字符串格式
        let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
        println!("{}", long_string);

        // 如果不想要转义可以使用\\ 来获得转义的\
        println!("{}", "hello \\x52\\x75\\x73\\x74");
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);

        // 如果字符串包含双引号，可以在开头和结尾加 #
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果还是有歧义，可以继续增加，没有限制
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);
    }
}
