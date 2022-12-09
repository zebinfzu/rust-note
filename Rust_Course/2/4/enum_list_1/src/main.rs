// 枚举实现的不带头节点的单向链表
enum List {
    Cons(u32, Box<List>),
    Nil,
}

use List::*;
impl List {
    fn new() -> List {
        Nil
    }
    // 头部插入一个节点
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }
    // 求链表的长度
    fn len(&self) -> usize {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}
