#[allow(unused)]
fn main() {
    // 1. rust最常见的智能指针Box
    // Box<T>可以允许将一个值分配到堆上，然后在栈上保存一个指向堆上数据的指针
    // rust中限制了main线程的栈是8MB, 一般线程的栈是2MB
    // 堆内存通常只受物理内存限制
    // 相比其它语言，Rust 堆上对象还有一个特殊之处，它们都拥有一个所有者，因此受所有权规则的限制

    // 2. 使用box的理由
    {
        // 特意要分配到堆上的时候
        // 数据比较大，不想在转移所有权的时候发生数据拷贝
        // 使用DST
        let a = Box::new(3);
        // println! 可以正常打印出 a 的值，是因为它隐式地调用了 Deref 对智能指针 a 进行了解引用
        println!("a = {}", a);
        // 下面一行代码将报错
        // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`

        let arr = [0; 1000];
        let arr1 = arr; // 因为是栈上的数据，所以转移所有权的时候发生了copy
        println!("{:?} {:?}", arr, arr1); // 都有效吧

        // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
        let arr = Box::new([0; 1000]);
        // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
        // 所有权顺利转移给 arr1，arr 不再拥有所有权
        let arr1 = arr;
        println!("{:?}", arr1.len());
        // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
        // println!("{:?}", arr.len());

        // DST获取固定大小
        // enum List {
        //     Cons(i32, List),
        //     Nil
        // }
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        // 利用特征对象存储不同类型元素的数组
        trait Draw {
            fn draw(&self);
        }
        let elems: Vec<Box<dyn Draw>> = vec![];
    }

    // 2. Box内存布局
    {
        let arr = vec![Box::new(1), Box::new(2)];
        // 借用数组元素，否则会报错
        let (first, second) = (&arr[0], &arr[1]);
        // 表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，第一次将 &Box<i32> 类型转成 Box<i32>，第二次将 Box<i32> 转成 i32
        let sum = **first + **second;
    }

    // 3. Box::leak 强制消费掉Box并且强制目标值从内存中泄漏
    {
        // 可以用在想把一个String类型变成'static周期的&str
        fn gen_static_str() -> &'static str {
            let mut s = String::new();
            s.push_str("hello, world!");
            Box::leak(s.into_boxed_str())
        }
        // 标注的 'static 只是用来忽悠编译器的，但是超出作用域，一样被释放回收。而使用 Box::leak 就可以将一个运行期的值转为 'static

        // 实际场景：需要一个在运行期初始化的值，但是可以全局有效，和程序活的一样久，就可以使用Box::leak，例如有一个存储配置结构的结构体实例，是运行期动态插入的，就可以通过这种方式变成全局有效
    }
}
