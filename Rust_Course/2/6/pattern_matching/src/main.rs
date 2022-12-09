#[allow(unused)]
fn main() {
    // 1. rust模式匹配是一种特殊语法，用来匹配类型中的结构和数据
    // 模式通常可以包含
    // 字面值
    // 解构的数组、枚举、变量、元组
    // 变量
    // 通配符
    // 占位符
    // 2. 最常用的match表达式，每一个分支就是一个模式
    {
        // match VALUE {
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        // }
    }
    // 3. while let 循环
    {
        // 只要模式匹配就会一直执行
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    // 3. for循环
    {
        let v = vec!['a', 'b', 'c'];
        // enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    // 4. let 语句
    {
        // let PATTERN = EXPRESSION;
        // 变量绑定语句也是一种模式匹配
        let x = 5;
        let (x, y, z) = (1, 2, 3);
    }
    // 5. 函数参数
    // 6. if let表达式可以非穷尽匹配，因此认为是可驳模式匹配
}
