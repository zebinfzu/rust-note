fn main() {
    // 所有权:
    // Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    // 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    // 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

    // 1. 字符串字面值会被硬编码到程序里面，类型&str，不可变
    let _s = "hello";
    // 2. 需要动态的字符串，存储在堆上可变的需要String类型
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s); // 将打印 `hello, world!`

    // 3. 转移所有权
    // 理解rust的等号是数据绑定，和别的语言的赋值有区别
    let x = 5; // 将5绑定到变量x，也称作x具有5的所有权
    let y = x; // 对于简单类型的值，rust会自动拷贝，即在栈上拷贝出一个新的5值，让y具有其所有权
               // y = x这种拷贝是栈上已知大小的内存快速复制，因此也算是浅拷贝

    // 对于非基本类型(存储在栈上)，rust不会自动拷贝
    // String是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成
    // 堆指针是最重要的，它指向了真实存储字符串内容的堆内存
    // 对于其他语言，比如js，s2 = s1，就会拷贝指针，s1，s2就会指向同样的堆地址
    // 对于rust则不然，堆内存同样同时只能有一个所有者，因此s2 = s1之后，s1就不再具有堆内存的所有权
    let s1 = String::from("hello");
    let s2 = s1;
    // 报错，编译器会提示s1是无效的引用，因为在让渡了所有权之后使用了s1
    println!("{}, world!", s1);
    // 因此在rust当中 s2 = s1这种行为不能被称作浅拷贝(shallow copy)，而要称为移动(move)

    // 再来看对于&str
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y); // 可以正常打印，编译器不会报错
                             // 在 String 的例子中 s1 持有了通过String::from("hello") 创建的值的所有权
                             // x 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权
                             // let y = x 中，仅仅是对该引用进行了拷贝，此时 y 和 x 都引用了同一个字符串，这里可以称为浅拷贝

    // 深拷贝deep copy
    // Rust永远不会自动创建深拷贝
    // 手动调用String的clone方法
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 可以浅拷贝(copy)的规则
    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的
    // 1. 整数、布尔值、浮点数、字符类型char
    // 2. 元组，仅当包含类型也都是可以Copy的类型时(i32, i32)
    // 3. 不可变引用&T

    // 函数传值与返回一样会发送move和copy
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x

    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}
