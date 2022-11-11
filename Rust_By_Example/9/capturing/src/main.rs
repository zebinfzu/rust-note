use std::mem;
fn main() {
    // 闭包可以捕获周围环境的变量
    // capture既可以move,又可以borrow
    // 通过引用 &T
    // 通过可变引用 &mut T
    // 通过值 T

    // 优先通过引用
    {
        let color = String::from("green");
        let print = || println!("`color`: {}", color);
        // 使用借用来调用闭包 `color`。
        print();

        // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
        let _reborrow = &color;
        print();

        // 在最后使用 `print` 之后，移动或重新借用都是允许的。
        let _color_moved = color;

        let mut count = 0;
        // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
        // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
        // 该闭包立即借用 `count`。
        //
        // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
        // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // 使用可变借用调用闭包
        inc();

        // 因为之后调用闭包，所以仍然可变借用 `count`
        // 试图重新借用将导致错误
        // let _reborrow = &count;
        // ^ 试一试：将此行注释去掉。
        inc();

        // 闭包不再借用 `&mut count`，因此可以正确地重新借用
        let _count_reborrowed = &mut count;

        // 不可复制类型（non-copy type）。
        let movable = Box::new(3);

        // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
        // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
        // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };

        // `consume` 消耗了该变量，所以该闭包只能调用一次。
        consume();
    }
    // | 之前使用move关键字强制闭包获取捕获变量的所有权
    {
        let haystack = vec![1, 2, 3];

        let contains = move |needle| haystack.contains(needle);
        println!("{}", contains(&1));
        println!("{}", contains(&4));

        //println!("There're {} elements in vec", haystack.len()); 报错，ownership移动了
    }
}
