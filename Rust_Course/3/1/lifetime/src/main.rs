#![allow(unused)]
fn main() {
    // 1. 生命周期：引用的有效作用域
    {
        let r;
        {
            let x = 5;
            r = &x;
        } // 到这里x被drop，因此引用了x的r也就无效了
          // 如果要在这里使用r编译器就会报错
          // println!("{}", *r); x的活的不足够久
    }
    // 2. 函数中的生命周期
    {
        // 报错，因为不知道参数和返回值的生命周期
        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }
    }

    // 3. 生命周期的标注语法
    // &i32 引用
    // &'a i32 具有显式生命周期的引用
    // &'a mut i32 具有显式生命周期的可变引用

    // 4. 标准生命周期让2中的函数合法
    {
        // 生命周期标准声明在泛型参数列表<>里面
        // 这里表示x,y有相同的生命周期，也就是说会和实参调用的时候活的最短的那个参数相同
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string_1 = String::from("long string is long");
        {
            let string_2 = String::from("xyz");
            let result = longest(string_1.as_str(), string_2.as_str());
            println!("{:?}", result); // 这里和法，string_1和string_2都还活着
        }
        // println!("{}", result); 这里不合法，虽然string_1还活着，但是返回值或者的长度只能和string_2一样
    }

    // 5. 因此可以知道一个函数的返回值如果是一个引用，那么只有可能有两种来源，要么他的生命周期来自于参数，要么就是函数体内新建数据的引用
    // 由此可以知道如果是函数体内新建数据的引用就是悬垂引用的问题，这种情况就不应该返回引用，而是将数据的所有权返回出去

    // 6. 结构体中的生命周期
    {
        // 同样结构体中的引用也要标准生命周期
        #[derive(Debug)]
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago ...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        // 结构体中引用的字符串必须要比结构体活的久
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        // 无法通过编译的例子
        let i;
        {
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            i = ImportantExcerpt {
                part: first_sentence,
            };
        }
        // 在novel死之后再去访问，就会导致编译器报错
        // println!("{:?}", i);
    }

    // 7. 编译器会函数自动推导生命周期，推不出来就会报错
    // 自动推导会依据以下三条原则
    // 每一个引用参数自动获得独自的生命周期
    // 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期传给所有输出生命周期
    // 若存在多个输入生命周期，且其中一个是&self或者&mut self,则&self生命周期会传给所有输出生命周期

    // 8. 方法中的生命周期
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        // 为具有生命周期的结构体实现方法，和泛型参数语法类似
        // impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
            // 方法经常不需要标注生命周期，因为三条消除原则
            // 不标准会默认和&self一致
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }

            // 但是这么干就会编译器报错，因为self的生命周期是'a, 返回的self.part也是'a，因此编译器需要知道'a和'b的关系
            // fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str {
            //     println!("Attention please: {}", announcement);
            //     self.part
            // }
        }

        // 实现的时候就要做约束， 'a 被约束为 'a >= 'b 活的比'b久
        impl<'a: 'b, 'b> ImportantExcerpt<'a> {
            fn announce_and_return_part_(&'a self, announcement: &'b str) -> &'b str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }

    // 9. 静态生命周期
    {
        let s: &'static str = "活的很久"; // 硬编码到二进制文件中
    }

    // 10. 一个复杂函数的例子，包含泛型、特征对象、生命周期
    {
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
