#![allow(unused)]
fn main() {
    use std::collections::HashMap;
    // 1. 创建哈希表
    {
        let mut scores = HashMap::new();
        // 哈希表中的key和value一样遵循所有权原则
        scores.insert(String::from("Blue"), 10);
    }
    // 2. 使用哈希表
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        // get方法，传入一个key的引用，返回一个value类型的Option
        let score = scores.get(&team_name);
        // 更新原有的值
        scores.insert(String::from("Blue"), 25);
        // 当没有值的时候才会插入
        scores.entry(String::from("Blue")).or_insert(50);
    }
}
