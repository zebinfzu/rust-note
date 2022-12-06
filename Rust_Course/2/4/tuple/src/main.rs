#[allow(unused)]
fn main() {
    // 1. 元组: (任意类型的组合)
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    // 2. 通过模式匹配解构元组
    {
        let tup = (1, 2, 3);
        let (x, y, z) = tup;
    }
    // 3. 使用.访问元组
    {
        let x = (500, 6.4, 1);
        let five_hundred = x.0;
    }
    // 4. 通常使用元组来作为函数要返回多个值的场景
    {
        fn calculate_length(s: String) -> (String, usize) {
            let len = s.len();
            (s, len)
        }

        let s1 = String::from("hello, world");
        let (s1, len) = calculate_length(s1);
        println!("{s1} length is {len}");
    }
}
