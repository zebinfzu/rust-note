#![allow(unused)]
fn main() {
    // lifetime 准确应该是寿命，生命周期感觉含有死了又可以活的歧义

    // lifetime 是rust中用以保证引用有效的机制(至少不能是使用引用的时候，引用的东西已经被释放了吧)

    // 1. lifetime -> 避免悬垂引用
    {
        let r;
        {
            let x = 5; // x 活了
            r = &x; // 只有这一行是合法的
        } // x 死了，这时候按照寿命，引用x的r应该也挂掉

        //  println!("{}", r); // 报错，因为编译器发现了一个对已经drop的变量的引用
    }

    // rust 的编译器有一个借用检查器(borrow checker)，用来比较作用域，以保证所有的借用是有效的

    // 2. 必须保证借用的的end time <= 原值的end time
    {
        let x = 5; // 'a
        let r = &x; // 'b
        println!("r: {}", r);
    } // 'a, 'b

    // 生命周期注解的写法(注意是引用才有，不要给正常的变量添加生命周期标注)，而且是泛型参数的时候才允许申明
    {
        // &i32 -> 引用
        // &'a i32
        // &'a mut i32
    }

    // 3. 函数中的生命周期
    {
        // 返回值有一个借用的时候，必须要注明生命周期
        // 至少我们要保证返回的值不能是出了函数就死了，那就是个无效的引用
        // 下面的标准说明，泛型参数给出一个生命周期的标记
        // 'a 表示传入的引用的x, y的生命周期为'a(含义是会取实际x,y原值中活得短的那一个), 而返回的引用活的时间和传入的引用的原值相同
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            // 只到这里还能用，因为这里string1, string2都活着
            result = longest(string1.as_str(), string2.as_str());
        }
        // 到这里编译使用result会报错，因为编译器会发现string2活的不够久， 函数longest返回的引用的寿命是按比较短的string2来的
        // println!("The longest string is {}", result);
    }

    // 4. 结构体中的lifetime
    {
        // 对于结构体中字段使用引用，同样要给出lifetime，以保证合法性
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
    }

    // 5. lifetime省略
    {
        // 返回值是引用，但是没有lifetime label一样可以通过编译
        // 因为编译器足够聪明，推的出来和参数是同样的lifetime
        // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }

    // 5. 方法当中的生命周期
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        impl<'a> ImportantExcerpt<'a>  {
            fn level(&self) -> &'a str {
                self.part
            }
        }
    }

    // 6. 静态生命周期
    // 'static，其生命周期能够存活于整个程序期间。所有的字符串字面量都拥有 'static 生命周期
    {
        // 手动标注
        let s: &'static str = "I have a static lifetime.";
    }
}
