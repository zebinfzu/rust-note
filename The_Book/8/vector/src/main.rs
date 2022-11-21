use std::vec;

fn main() {
    // 1. rust中数组是固定长度的，要创建动态数组需要使用vector
    {
        // 使用new运算符，上下文没有办法推导元素类型的时候才需要注明类型
        let _v: Vec<i32> = Vec::new();
        // 使用vec![]宏
        let _v = vec![1, 2, 3, 4];
    }
    // 2. 更新vector
    {
        let mut v = Vec::new();
        // push 追加元素
        for i in 0..10 {
            v.push(i);
        }
        println!("{:?}", v);
        // insert 插入元素，其他元素右移
        v.insert(1, 4);
        // remove 删除元素，其他元素左移
        v.remove(0);
        // pop 弹出最后一个元素
        v.pop();
    }
    // 3. 当vector drop的时候，里面的元素也会drop
    // 4. 访问元素方法
    {
        let v = vec![1, 2, 3, 4, 5];
        let _third = &v[2]; // 使用下标访问，越界会导致panic

        // .get方法不会panic,而是返回一个Option值
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }
    // 5. 遍历vector中的元素
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        for (idx, val) in v.iter().enumerate() {
            println!("{} {}", idx, val);
        }
    }
    // 6. 利用枚举来存储多种类型的值
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
