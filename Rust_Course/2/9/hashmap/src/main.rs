#[allow(unused)]
fn main() {
    // 1. rust中的哈希表没有自动引入，需要手动引入
    use std::collections::HashMap;
    let mut my_gems = HashMap::new();
    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    // 2. 通过数组元组创建哈希表
    {
        let teams_list = vec![
            ("中国队".to_string(), 100),
            ("美国队".to_string(), 10),
            ("日本队".to_string(), 50),
        ];

        let mut teams_map = HashMap::new();
        for team in &teams_list {
            teams_map.insert(&team.0, team.1);
        }

        println!("{:?}", teams_map)
    }

    // 3. 通过key查询value方法get
    // 返回的是值引用类型的Option
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
    }

    // 4. 更新hashmap中的值
    {
        let mut score = HashMap::new();
        score.insert("Blue", 10);
        // 覆盖原有的值，会返回旧的值
        let old = score.insert("Blue", 20);
        assert_eq!(old, Some(10));

        // 查询key对应的值，如果不存在则插入值，已存在则不会插入
        let v = score.entry("Yellow").or_insert(5);
        // 这个方法返回的是插入的值的可变引用
        *v = 10;
        println!("{:?}", score);
    }
}
