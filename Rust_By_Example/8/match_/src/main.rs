#[allow(unused)]
fn main() {
    // match 必须是穷尽匹配的
    // 元组
    {
        let triple = (0, -2, 3);
        match triple {
            (0, y, z) => (),
            (1, ..) => (),
            _ => (),
        }
    }
    // enum
    {
        #[allow(dead_code)]
        enum Color {
            // 这三个取值仅由它们的名字（而非类型）来指定。
            Red,
            Blue,
            Green,
            // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }
        let color = Color::RGB(122, 17, 40);
        // 试一试 ^ 将不同的值赋给 `color`

        println!("What color is it?");
        // 可以使用 `match` 来解构 `enum`。
        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) => println!(
                "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k
            ),
            // 不需要其它分支，因为所有的情形都已覆盖
        }
    }
    // 指针和引用
    {
        // 获得一个 `i32` 类型的引用。`&` 表示取引用。
        let reference = &4;

        match reference {
            // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
            // `&i32`（译注：即 `reference` 的类型）
            // `&val`（译注：即用于匹配的模式）
            // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
            // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
            &val => println!("Got a value via destructuring: {:?}", val),
        }
        // 如果不想用 `&`，需要在匹配前解引用。
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }

        // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
        // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
        let _not_a_reference = 3;

        // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
        // 下面这行将得到一个引用。
        let ref _is_a_reference = 3;

        // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
        let value = 5;
        let mut mut_value = 6;

        // 使用 `ref` 关键字来创建引用。
        // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
        // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
        // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
        // 引用。
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        // 类似地使用 `ref mut`。
        match mut_value {
            ref mut m => {
                // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
                *m += 10;
                println!("We added 10. `mut_value`: {:?}", m);
            }
        }
    }
    // 卫语句
    {
        let pair = (2, -2);
        // 试一试 ^ 将不同的值赋给 `pair`

        println!("Tell me about {:?}", pair);
        match pair {
            (x, y) if x == y => println!("These are twins"),
            // ^ `if` 条件部分是一个卫语句
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }
    // 绑定
    // 在 match 中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。match 提供了 @ 符号来绑定变量到名称
    {
        let age = 1;
        match age {
            0 => println!("0"),
            n @ 1..=12 => println!("{}", n),
            _ => (),
        }
        // 通过绑定解构枚举的变体
        let some_num = Some(42);
        match some_num {
            Some(n @ 0..=4) => println!("{}", n),
            Some(n) => (),
            _ => (),
        }
    }

    // if let 代替 match的用法
    {
        // 将 `optional` 定为 `Option<i32>` 类型
        let optional = Some(7);

        match optional {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i);
                // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
                // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
            }
            _ => {} // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
        };

        // if let 可以是非穷尽的
        if let Some(i) = optional {
            println!("This is a really long string and `{:?}`", i);
        }

        // 用来匹配枚举的所有值
        // 以这个 enum 类型为例
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }
        // 创建变量
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        // 变量 a 匹配到了 Foo::Bar
        if let Foo::Bar = a {
            println!("a is foobar");
        }

        // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
        if let Foo::Bar = b {
            println!("b is foobar");
        }

        // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
        if let Foo::Qux(value) = c {
            println!("c is {}", value);
        }
        // if let 允许匹配枚举非参数化的变量，即枚举未注明 #[derive(PartialEq)]，我们也没有为其实现 PartialEq。在这种情况下，通常 if Foo::Bar==a 会出错，因为此类枚举的实例不具有可比性。但是，if let 是可行的
    }

    {
        enum Foo {
            Bar,
        }
        let a = Foo::Bar;

        // 变量匹配 Foo::Bar
        // if Foo::Bar == a {
        //     // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
        //     println!("a is foobar");
        // }

        if let Foo::Bar = a {
            // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
            println!("a is foobar");
        }
    }

    // while let
    {
        let mut optional = Some(0);
        loop {
            match optional {
                // 如果 `optional` 解构成功，就执行下面语句块。
                Some(i) => {
                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    } else {
                        println!("`i` is `{:?}`. Try again.", i);
                        optional = Some(i + 1);
                    }
                    // ^ 需要三层缩进！
                }
                // 当解构失败时退出循环：
                _ => {
                    break;
                }
            }
        }
        // 使用while let
        while let Some(i) = optional {
            if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
        }
    }
}
