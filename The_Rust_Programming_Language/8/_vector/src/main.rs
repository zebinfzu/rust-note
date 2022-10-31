#![allow(unused)]
fn main() {
    {
        // 新建vector
        //  1. new 关联函数
        let v: Vec<i32> = Vec::new();
        // 2. !vec[]宏
        let mut v = vec![1, 2, 3];
        // 更新vector
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    } // 作用域结束，v被drop，堆上的内存回收
    {
        // 读取vector
        let v = vec![1, 2, 3, 4, 5];
        // 1. 索引语法
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        // 2. get方法
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
        // 两种方法的区别
        // 索引会出现越界时导致程序panic的问题
        // get方法则返回Option<T>，因此不会导致程序panic，越界访问会返回None
    }
    {
        // 遵循移动，拷贝，复制的规则
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0]; // 创建一个不可变引用

        // v.push(6); 报错，因为push方法会获取本身的可变引用，不可以同时使用不可变引用和可变引用

        println!("The first element is: {}", first); // first使用一次就不能再用了

        v.push(6);
    }
    {
        // 遍历
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        // 遍历的同时要修改值
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }
    {
        // vector只能存储一种类型的值
        // 使用枚举使得vector能存储多种类型的值
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        // 这种方式适合于编译的时候就知道vector会存在多种类型，可以使用match处理所有不同类型的情况
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        // 如果需要运行时才知道元素确切类型的，枚举就没办法了，需要使用trait对象
    }
}
