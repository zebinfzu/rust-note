#![allow(unused)]
fn main() {
    // ownership
    {
        let s = "hello";
    } // 不再有效，作用域原则
    // 对于rust来说，栈上的数据，或者说实现了Copy特征的数据，=发生的是复制
    // 而对于堆上的数据，=发生的是所有权的移动
    {
        let x = 1;
        let y = x; // 发生copy
        let z = x + y; // x还可以使用
        // String是一个复杂类型，在栈上存储了对指针、字符串长度、字符串容量
        let s1 = "hello".to_string();
        let s2 = s1; // 发生了移动，之后s1不再可以使用
        // let s3 = s2.to_string() + &s1; 报错，不可以借用已经移动了的变量
    }
    // 深拷贝clone，即堆上数据的复制
    // 对于String有clone方法
    {
        let s1 = "hello".to_string();
        let s2 = s1.clone();
    }
    // 浅拷贝(copy)，可以发生浅拷贝的类型：基本类型(数值、布尔、字符、只包含可以浅拷贝类型的元组、不可变引用&T)
    // 注意函数的返回值和传参一样依据所有权的原则
}

