#[allow(unused)]
fn main() {
    // 1. char类型固定4byte，支持unicode字符
    {
        let c = 'z';
        let z = 'ℤ';
        let g = '国';
        let heart_eyed_cat = '😻';
    }
    // 2. boolean 1byte
    {
        let a = true;
        let b = false;
    }
    // 3. unit
    // 类型为(),唯一的值也是unit
    {
        // 函数没有返回值就会默认返回单元类型()
        // 语句可以认为是返回单元类型
        fn no_return() -> () {}
    }
}
