#[allow(unused)]
fn main() {
    // 1. ownership 所有权是rust独有的概念
    /**
     * 关于编程语言如何处理内存，即啥时候申请，啥时候释放，现在主流的处理方法分成3种流派
     * 垃圾回收机制(GC Garbage Collection)，典型代表: Java、Go、Javascript
     * 手动管理申请和释放：C++
     * 所有权规则：编译期间检查 rust
     */
    // 一段c的代码，这段代码可以通过c的编译器，但是局部变量a在函数结束后，栈上申请的内存就被回收了，因此返回的指针就成了个不安全的悬垂指针
    // int* foo() {
    //     int a;          // 变量a的作用域开始
    //     a = 100;
    //     char *c = "xyz";   // 变量c的作用域开始
    //     return &a;
    // }                   // 变量a和c的作用域结束

    // 2. 所有权规则
    // Rust中每一个值都被一个变量拥有，该变量称为值的所有者
    // 一个值同时只能被一个变量拥有，或者说一个值只能有一个所有者
    // 当所有者离开作用域的时候，将会将值丢弃
    {
        // &str是字符串字面值，是不可变的，硬编码进去的
        let s = "hello";
    } // s 的作用域到此位置，值被丢弃

    // 3. 转移所有权
    // rust当中=更准确的称呼应该是绑定而非赋值，当使用=的时候会发生的情况
    // Copy 实现了Copy特征的类型或者primitive type将会发生值的复制，栈上值的快速复制
    // Clone 实现了Clone特征的类型，深拷贝，拷贝堆上数据，栈上的值是不一样的
    // move 移动，发生了所有权的移动，原来的所有者将不能再被使用 其实是浅拷贝，拷贝了原来栈上的数据
    {
        // String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量
        let s1 = String::from("hello");
        // move s2复制了s1栈上的内容 浅拷贝 shallow copy
        let s2 = s1; // s1不能再被使用
                     // println!("s1"); // 报错，在移动之后使用了变量
    } // 离开作用域的时候，会认为s1已经失效了，所以不会drop s1 避免double free的问题

    // 4. 借用可以使用值，但是没有原数据的所有权
    {
        let x = "hello world"; // 这是一个借用，x没有"hello world"的所有权
        let y = x; // 发生了copy，y也是借用，栈上数据快速复制
        println!("{}{}", x, y);
    }

    // 5. 深拷贝 Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
    }

    // 6. 函数传值同样会发生了Copy 和 move
    {
        let s = String::from("hello"); // s 进入作用域

        takes_ownership(s); // s 的值移动到函数里 ...
                            // ... 所以到这里不再有效

        let x = 5; // x 进入作用域

        makes_copy(x); // x 应该移动函数里，~
                       // 但 i32 是 Copy 的，所以在后面可继续使用 x

        fn takes_ownership(some_string: String) {
            // some_string 进入作用域
            println!("{}", some_string);
        } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

        fn makes_copy(some_integer: i32) {
            // some_integer 进入作用域
            println!("{}", some_integer);
        } // 这里，some_integer 移出作用域。不会有特殊操作，栈空间自己回收
    }
}
