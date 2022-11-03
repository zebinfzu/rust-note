#![allow(unused)]
fn main() {
    // 切片
    {
        // 创建语法 集合[开始索引..结束索引]
        let s = String::from("abcdefg");
        // 不要用range创建字符串切片，因为每个字符的字节长度不一定相等
        let slice = &s[..]; // 引用整个String
                            // 字符串字面量是切片
        let s = "hello";
        // 其他集合一样有切片，例如数组
        let a = [1, 2, 3, 4, 5];
        // 切片类型是&[t]
        let b = &a[1..3];
        // 切片的长度在编译期不知道，因此不能直接使用如str,[i32]这样的类型，只能使用引用&str, &[i32]
        let arr = ['索', '多', '玛'];
        let slice = &arr[..2];
        // 因为是切片引用而不是数组
        assert!(std::mem::size_of_val(&slice) == 16);
    }
    // 什么是字符串
    {
        // 如何将String转换到&str
        // 取引用
        // 不要创建
        let s = String::from("kkkkk");
        let a = &s;
        let b = &s[..];
        let c = s.as_str();
        // mut String 的一些常用方法
        // push 追加
        // insert
        // replace replacen replace_range
        // delete
        // 链接字符串使用 + += 左操作数是String，右操作数是&str，该方法会移动左操作数的所有权
        // format!宏链接字符串，该宏不会拿走变量的所有权
        // 字符串使用""包裹，中间使用\来转义
        // 或者使用r#""#包裹字符串，当中可以使用"，但不会转义内容
        // 使用r###""###包裹字符串，当中可以使用"和#
        let t = "\x73\x74";
        let quotes = r#"And then I said: "There is no escape!""#;
        // 使用\可以链接多行
        let t = "hello \
        world";
        // 将字符串转换成char的迭代器，就可以使用for in访问到每一个字符
        let t = "你好，世界".chars();
        for i in t {
            println!("{i}");
        }
        // &String会被隐式转换为&str

        // String 的底层是 Vec<u8> 以字节数组的方式存储
        // &str 是切片引用类型 &[u8]
        // 底层String是一个智能指针，当前容量足够添加字符就不会导致重新分配内存
        let story = String::from("Rust By Practice");
        let mut story = std::mem::ManuallyDrop::new(story);

        let ptr = story.as_mut_ptr();
        let len = story.len();
        let capacity = story.capacity();

        println!("{:?} {len} {capacity}", ptr);
    }
}
