fn main() {
    // 1. encapsulation 封装
    // rust当中默认结构体、函数、方法、枚举等等都是模块私有的，只有加上了pub关键字的才可以被模块外访问
    #[allow(unused)]
    mod moo {
        // pub 表示外界可以访问这个数据结构
        pub struct AveragedCollection {
            list: Vec<i32>, // 外界不能访问该字段
            average: f64,
            pub classes: String, // 外界可以访问这个字段
        }
        impl AveragedCollection {
            pub fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }

            pub fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop();
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    }
                    None => None,
                }
            }

            pub fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }
    }

    // 2. inheritance 继承
    // rust不提供继承，使用继承的目的主要是：
    // 复用代码：子类可以重用父类的代码，这点rust可以很容易使用提供相同的默认特征就可以实现

    // 3. polymorphism 多态
    // 对于rust同理，要描述可以用于多种类型的更加广泛的概念，压根不需要像c++搞共同祖先不同派生类
    // rust只要对泛型限定必须实现的特征就可以达到多态的目的
}
