#[allow(unused)]
fn main() {
    // 1. 关联类型
    // 指的是在特征定义语句块中，申明一个自定义类型，这样就可以在特征的方法签名中使用该类型
    {
        // 标准库中迭代器特征Iterator，有一个Item关联类型，用于体改遍历的值的类型
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        // 为什么不使用泛型
        {
            pub trait Iterator<Item> {
                fn next(&mut self) -> Option<Item>;
            }
        }
        // 这是为了代码可读性，当所有地方都需要协商Iterator<Item>的时候可读性会很差，类型定义复杂的例子
        // 只写Address可比泛型写AsRef<[u8]> + Clone + fmt::Debug + Eq来的可读性强
        {
            pub trait CacheableItem: Clone + Default + std::fmt::Debug {
                type Address: AsRef<[u8]> + Clone + std::fmt::Debug + Eq;
                fn is_null(&self) -> bool;
            }
        }
        // 再比如
        {
            trait Container<A, B> {
                fn container(&self, a: A, b: B) -> bool;
            }
            fn difference<A, B, C>(container: &C) -> i32
            where
                C: Container<A, B>,
            {
                1
            }
        }
        // 使用关联类型则可读性强很多
        {
            trait Container {
                type A;
                type B;
                fn container(&self, a: &Self::A, b: &Self::B) -> bool;
            }
            fn difference<C: Container>(container: &C) -> i32 {
                1
            }
        }
    }
}
