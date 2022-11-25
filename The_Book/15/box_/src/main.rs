#[allow(unused)]
fn main() {
    // 1. Box创建堆上的数据
    {
        let b = Box::new(5);
        // 可以像数据是储存在栈上的那样访问 box 中的数据
        println!("{}", b); // 5
    } // b 被 drop，堆上分配的空间也释放

    // 2. Box可以创建递归数据类型
    // 由于rust中栈上的数据类型必须是编译时知道大小的，因此诸如链表这样的数据结构，在rust中实现比别的语言麻烦很多，因为可以无限嵌套，所以编译期是不知道能占多大空间的

    // cons list 单向链表：每一项包含两个字段：1. 当前项的值 2. 下一项
    {
        // 报错 因为List的字段中又包含了List，编译器不可能知道会有多大
        // enum List {
        //     Cons(i32, List),
        //     Nil
        // }
    }

    // 计算非递归类型的大小
    {
        // 枚举按最大的枚举成员
        enum Message {
            Quit,                       // -> 0
            Move { x: i32, y: i32 },    // -> 2 * i32(4byte)
            Write(String),              // -> String是智能指针，
            ChangeColor(i32, i32, i32), // 3 * i32(4byte)
        }
        // struct 按照所有字段
        struct A {
            x: i32,
            y: i32,
        }
    }

    // 因此递归类型是算不出来大小的

    // 3. 但是指针大小是固定的，因此可以使用指针来保存递归类型
    {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        use List::{Cons, Nil};
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
    // 或者简单的单向链表适用一个Option
    {
        // 定义节点
        struct ListNode {
            val: i32,
            next: Option<Box<ListNode>>,
        }
        let list = Some(Box::new(ListNode { val: 3, next: None }));
    }
}
