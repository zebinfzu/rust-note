fn main() {
    // if let语法用来处理只匹配一个模式的值而忽略其他模式
    {
        // 使用match
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maxium is configured to be {}", max),
            _ => (),
        }
    }
    {
        // 使用if let
        // 语法 if let 模式 = 表达式 {} 就是match的一个语法糖
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
        // 同时可以使用一个else来表示match中_的情况
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        } else {

        }
    }
}
