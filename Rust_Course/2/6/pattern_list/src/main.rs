#[allow(unused)]
fn main() {
    // 1. 匹配字面量
    {
        let x = 1;
        // match需要匹配所有该类型的值
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    // 2. 匹配变量命名
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }
    // 3. 单分支多模式 |
    {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    // 4. 通过序列 ..= 匹配范围值
    {
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }
    // 5. 使用模式来解构结构体，枚举，元组，数组，引用
    {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        // 解构的时候不给名字则变量名和字段相同
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }

        // 解构元组
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

        // 解构数组
        let arr: [u16; 2] = [114, 514];
        let [x, y] = arr;
    }
    // 6. 使用_忽略模式中的值
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }
        foo(3, 4);

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);
        // 嵌套的使用_
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);
        // 使用多处_忽略特定值
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }
    }
    // 7. 使用..忽略剩余值
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        // 用来忽略元组中间的值
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
        // .. 忽略内容不能有歧义，不然会导致报错
    }
    // 8. 匹配守卫，为匹配的值提供额外的匹配条件
    {
        let x: u8 = 10;
        match x {
            n @ 0..=100 if n % 2 == 0 => {
                println!("even in 0~100")
            }
            _ => (),
        }
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }
    // 9. @绑定
    {
        // 序列生成的值想要使用的时候可以使用@绑定到一个变量
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => {
                println!("Found an id in range: {}", id_variable)
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }
        }
    }
}
