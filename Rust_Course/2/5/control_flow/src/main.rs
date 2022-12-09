#[allow(unused)]
fn main() {
    // 1. rust的if else 和别的语言没什么区别，不过在rust中if else同样是表达式，{}内容会作为返回值
    {
        let num = if 10 % 2 == 0 { "even" } else { "odd" };
    }
    // 2. 由于if else 可以作为表达式使用，所以rust没提供?:表达式
    // 3. for循环
    // for 元素 in 元素集合
    {
        let range = 1..3;
        // 注意不使用引用，所有权会move进循环
        // 当然数组或者序列生成的可以Copy的元素会获得Copy
        for i in range {
            println!("{}", i);
        }

        let collection: Vec<String> = vec![String::from("1"), String::from("2")];

        // 被move进去
        for s in collection {
            println!("{}", s);
        }

        // 更合理应该变成迭代器
        let mut collection: Vec<String> = vec![String::from("1"), String::from("2")];

        for s in collection.iter() {
            println!("{}", s);
        }

        // 需要在循环过程中修改元素的值
        for s in collection.iter_mut() {
            s.push('E');
        }

        // 需要在遍历的同时获取索引
        for (index, s) in collection.iter().enumerate() {
            println!("{}, {}", index, s);
        }

        // 只是想要迭代，不需要多余变量
        for _ in 0..10 {
            println!("print line");
        }
    }

    // 4. continue 跳过本轮循环，break 结束当前循环
    // 5. while 循环，和别的语言没差
    // 6. loop 最通用的循环，和continue和break结合使用
    {
        let mut t = 0;
        // break可以跟上一个表达式作为loop表达式的返回值
        let n = loop {
            t += 0;
            if t > 10 {
                break t;
            }
        };
        // continue默认跳出最近的循环，也可以使用continue label语法来指定跳出的循环
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    // 这只会跳出 inner1 循环
                    break 'inner1; // 这里使用 `break` 也是一样的
                }
                count += 2;
            }

            count += 5;

            'inner2: loop {
                if count >= 30 {
                    break 'outer;
                }

                continue 'outer;
            }
        }

        assert!(count == 30)
    }
}
