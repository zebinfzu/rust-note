/// 这里给出一个“人”的表示
pub struct Person {
    /// 一个人必须有名字（不管 Juliet 多讨厌她自己的名字）。
    name: String,
}

impl Person {
    /// 返回具有指定名字的一个人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串切片，代表人的名字
    ///
    /// # 示例
    ///
    /// ```
    /// // 在文档注释中，你可以书写代码块
    /// // 如果向 `rustdoc` 传递 --test 参数，它还会帮你测试注释文档中的代码！
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    // 1. 单行注释
    /* 2. 块注释，进制嵌套 */
    // 3. ///文档注释，使用cargo doc可以构建文档，支持markdown语法
}
