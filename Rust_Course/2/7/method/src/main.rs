#[allow(unused)]
fn main() {
    // 1. rust可以给类型定义方法，不只是struct,enum，原始类型同样可以
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        impl Circle {
            // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
            // 这种方法往往用于初始化当前结构体的实例
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle {
                    x: x,
                    y: y,
                    radius: radius,
                }
            }

            // Circle的方法，&self表示借用当前的Circle结构体
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }
    }
    // 2. 给原始类型定义新方法
    {
        // 没法直接添加，需要通过特征
        trait Current {
            fn current_print(&self);
        }
        impl Current for i32 {
            fn current_print(&self) {
                println!("current: {}", self);
            }
        }
        let i = 30;
        i.current_print();
    }
    // 3. method方法的参数语法糖
    {
        // 在impl块中Self表示类型本身
        // 在impl块中的方法的第一个参数
        // self表示类型实例被move进这个方法
        // &self表示获取类型实例的不可变引用
        // &mut self表示获取类型实例的可变引用
        // 方法名允许和字段名字相同
        struct Rect {
            width: u8,
            height: u8,
        }
        impl Rect {
            fn new(width: u8, height: u8) -> Self {
                Rect { width, height }
            }
            // 这种方式通常是用来实现getter访问器
            // 对于本mod和同级文件外的代码，结构体默认字段均为私有，不允许访问
            // 因此可以使用和字段同名的方法作为pub方法来返回值
            pub fn width(&self) -> u8 {
                self.width
            }
        }
    }
    // 4. 关联函数
    {
        // impl块中函数的第一个参数不是self, &self, &mut self的被称为关联函数
        // 这种函数无法使用实例.函数名来调用
        // 只能使用类名::函数名来调用
        // 最常见的就是类型的new方法
    }
}
