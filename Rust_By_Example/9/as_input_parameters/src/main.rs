use std::vec;

fn main() {
    // 闭包作为函数参数的时候，必须完整指出闭包的类型，需要通过指定一种trait的一种
    // Fn: 表示捕获方式通过引用
    // FnMut
    // FnOnce
    {
        fn apply<F>(f: F)
        where
            F: FnOnce(),
        {
            f();
        }
        // 输入闭包，返回一个 `i32` 整型的函数。
        fn apply_to_3<F>(f: F) -> i32
        where
            // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        use std::mem;

        let greeting = "hello";

        let mut farewell = "goodbye".to_owned();

        // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
        let diary = || {
            // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
            println!("I said {}.", greeting);

            // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
            // 现在需要 `FnMut`。
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");

            // 手动调用 drop 又要求闭包通过值获取 `farewell`。
            // 现在需要 `FnOnce`。
            mem::drop(farewell);
        };
        // 以闭包作为参数，调用函数 `apply`。
        apply(diary);

        // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
        let double = |x| 2 * x;

        println!("3 doubled: {}", apply_to_3(double));
    }

    // 函数作为函数的参数
    {
        // 如果你声明一个接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数
        fn call_me<F: Fn()>(f: F) {
            f();
        }

        fn function() {
            println!("I'm a function!");
        }

        let closure = || println!("I'm a closure");
        call_me(closure);
        call_me(function);
    }
    {
        // 指定闭包参数的入参和返回值类型
        fn call_me<F>(f: F, a: i32, b: i32) -> i32
        where
            F: Fn(i32, i32) -> i32,
        {
            f(a, b)
        }
        println!("{}", call_me(|a, b| a + b, 1, 2));

        // 闭包作为返回值，要指定实现的特征，并且必须move
        fn create_fn() -> impl Fn() {
            let text = "Fn".to_owned();

            move || println!("This is a: {}", text)
        }

        fn create_fnmut() -> impl FnMut() {
            let text = "FnMut".to_owned();

            move || println!("This is a: {}", text)
        }

        fn create_fnonce() -> impl FnOnce() {
            let text = "FnOnce".to_owned();

            move || println!("This is a: {}", text)
        }

        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();

        fn_plain();
        fn_mut();
        fn_once();
    }
    // 标准库的迭代器find方法
    let vec1 = vec![1, 2, 3, 4, 5];
    println!("{}", vec1.iter().find(|&&x| x == 2).unwrap());
}
