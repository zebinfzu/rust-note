fn main() {
    // 1. rust中字符char类型是unicode占4个字节，且只能用单引号包裹
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
    // 2. boolean 1Byte
    let t: bool = true;
    // 3. 单元类型unit()，唯一值只有()，main函数返回值就是()，rust中没有返回值的函数被称为发散函数(diverge function)，占内存0
}
