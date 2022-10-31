#![allow(unused)]
// HashMap是三个常用集合中最不常用的，因此没有被prelude自动引入
use std::collections::HashMap;
fn main() {
    {
        // 新建哈希表
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        // 没插入值的情况下一样需要手动说明k-v的类型
        let hash: HashMap<String, i32> = HashMap::new();

        // 使用zip可以创建一个元组迭代器，collect 方法将这个元组的迭代器转换成一个 HashMap
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        // 所有权规则同理，对于栈上的i32或其他实现了Copy trait的类型会直接复制值给哈希表，而String这样的类型则会发生所有权的移动
        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    }
    {
        // 访问hash_map中的值
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        // 会返回Option<&T>
        let score = scores.get(&team_name);
        // 覆盖一个值
        scores.insert(String::from("Blue"), 25);
        // 只有没有值的时候才插入
        // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
        scores.entry(String::from("Blue")).or_insert(50);
    }
    {
        // 根据旧的值来更新一个值
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
