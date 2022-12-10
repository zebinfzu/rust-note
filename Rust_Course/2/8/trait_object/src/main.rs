#[allow(unused)]
fn main() {
    // 1. rust特征无法解决的问题
    // 其他语言希望多态的时候会使用一个基类作为参数类型，使得传入的子类实参均合法有效
    // rust不存在继承，同时因为严格的类型检查，这样的行为行不通
    // 希望传入的参数的一个泛型数组，这个T[]，这个T有约束必须实现某一个特征，但实际上T也只能是一种具体类型，而不是可以是多种实现了同一特征的不同类型
    // 为了解决这个问题，实现函数或者其他使用情况可以使用实现同一特征但类型不同的方式，可以有一下几种
    // 2. 通过枚举
    {
        // 既然不可以传入不同类型，那么通过枚举，就可以使得不同的类型变成同一类型了
        enum Animal {
            Cat,
            Dog,
        }
        let animals = [Animal::Cat, Animal::Dog];
        fn foo(arr: &[Animal]) {}
    }
    // 3. 特征对象
    {
        struct Dog;
        struct Cat;
        trait Say {
            fn say(&self) {
                println!("我是动物");
            }
        }
        impl Say for Cat {
            fn say(&self) {
                println!("我是猫");
            }
        }
        impl Say for Dog {
            fn say(&self) {
                println!("我是狗");
            }
        }
        // 注意这时候传递给函数的类型使用动态数组存储的时候还是只能是固定的一种类型
        // 但是可以使用特征对象，使用&或者Box指针可以创建特征对象
        // Box<dyn trait> 表示Box指针指向堆上一个实现了指定特征的对象
        // dyn表示这个分配过程是动态的，编译期不确定，是执行的时候才知道的
        let animals: Vec<Box<dyn Say>> = vec![Box::new(Cat), Box::new(Dog)];
        fn foo(animals: &[Box<dyn Say>]) {
            for animal in animals {
                animal.say();
            }
        }
        foo(&animals[..]);
    }

    // 4. 对于特征对象的要求
    // 其所有的方法返回类型不能是Self
    // 方法没有任何泛型参数
    // 典型例如Clone特征就不符合特征对象的要求，因为clone方法返回的是Self类型
}
