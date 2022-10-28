fn main() {
    // 1. rust当中if是表达式，有返回值
    let t = 10;
    let y = if t % 2 == 0 { 2 } else { 1 };
    println!("{y}");

    // 2. loop循环 同样是表达式，使用break跳出loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // 多层嵌套的loop可以使用循环标签(loop label)来消除break和continue的歧义
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("{count}");

    // 3. while循环
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // 4. for遍历集合，可以消除while通过下标访问越界的隐患
    let a = [100, 111, 12, 3];
    for element in a {
        println!("{element}");
    }

    // for 要执行特定次数，可以使用Range方法来生成序列
    for number in (1..4).rev() {
        println!("{number}");
    }
}
