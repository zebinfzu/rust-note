fn main() {
    // 1. if expression
    // unlike languages such as ruby and javascript, rust will not automatically try to convert non-Boolean type to a boolean.
    {
        let number = 5;
        let _number = if number % 2 == 0 { 0 } else { 1 };
    }
    // 2. loop while for
    {
        // 无限的循环，直到break
        // loop {
        //     println!("again");
        // }
        // 'label:, break 和 continue 可以跟上 label来指明是哪个loop
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;
            loop {
                println!("remaining = {}", remaining);
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
        println!("End count = {}", count);

        // break 可以跟一个值作为loop expression的返回值
        let number = 'another: loop {
            count += 1;
            if count < 30 {
                count %= 20;
                count += 18;
            } else {
                break 'another count;
            }
        };
        println!("number: {}, count: {}", number, count);
    }

    // while 和 if 一样需要一个返回值为boolean的表达式作为条件
    {
        let mut count = 5;
        while count == { count % 2 } {
            if count > 10 {
                count /= 10;
            } else {
                count += 5;
            }
        }
    }

    // for 用来迭代集合
    {
        let a = [10, 20, 30, 40, 50];
        for c in a {
            println!("the value is: {}", c);
        }

        let a = vec![10, 20, 30, 40, 50];
        for c in a.iter() {
            println!("the value is: {}", c);
        }
    }
}
