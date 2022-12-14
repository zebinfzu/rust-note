use std::collections::HashMap;

#[allow(unused)]
fn main() {
    // 1. for循环和迭代器
    {
        let arr = vec![1, 2, 3];
        for v in arr {
            println!("{}", v);
        }
        // 数组实现了IntoIterator 特征，Rust 通过 for 语法糖，自动把实现了该特征的数组类型转换为迭代器
        // IntoIterator 特征拥有一个 into_iter 方法，因此我们还可以显式的把数组转换成迭代器
        let arr = vec![1, 2, 3];
        for v in arr.into_iter() {
            println!("{}", v);
        }
    }
    // 2. 惰性初始化
    {
        let v1 = vec![1, 2, 3];
        // 创建了一个v1的迭代器，但是不使用的话就什么都不会发生
        let v1_iter = v1.iter();
        // 语法糖，实际上是通过next方法取出迭代器中的值
        for val in v1_iter {
            println!("{}", val);
        }

        let arr = [1, 2, 3];
        let mut arr_iter = arr.into_iter();
        // next 方法返回的是 Option 类型
        // next 方法对迭代器的遍历是消耗性的，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 None
        assert_eq!(arr_iter.next(), Some(1));
        assert_eq!(arr_iter.next(), Some(2));
        assert_eq!(arr_iter.next(), Some(3));
        assert_eq!(arr_iter.next(), None);
    }

    // 3. for是语法糖，通过迭代器模拟实现for
    {
        let values = vec![1, 2, 3];
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => println!("{}", x),
                    None => break,
                }
            },
        };
        result
    }

    // 4. 迭代器自身也实现了IntoIterator特征
    {
        let values = vec![1, 2, 3];

        for v in values.into_iter().into_iter().into_iter() {
            println!("{}", v)
        }
    }

    // 5. into_iter, iter, iter_mut
    // into_iter将拿走所有权
    // iter获取数据的不可变借用
    // iter_mut 获取数据的可变借用

    // Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 next
    // IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器

    // 6. 消费者、适配器
    {
        // 只要迭代器上的某个方法 A 在其内部调用了 next 方法，那么 A 就被称为消费性适配器
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum(); // 拿走迭代器的所有权，方法第一个参数是self
        assert_eq!(total, 6);
        // v1_iter 是借用了 v1，因此 v1 可以照常使用
        println!("{:?}", v1);

        // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
        // println!("{:?}",v1_iter);

        // 既然消费者适配器会消耗迭代器，自然就不能链式调用
        // 迭代器适配器则会返回一个新的迭代器

        let v1: Vec<i32> = vec![1, 2, 3];
        // map是一个迭代者适配器是惰性的，实际上什么也不会被立即执行
        v1.iter().map(|x| x + 1);
        // 因此要获取map的效果，就需要再来一个消费者适配器收尾
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        // 为何 collect 在消费时要指定类型？
        // 因为collect本身很强大，可以收集成不同的集合类型，所以需要告诉编译器要收集成Vector

        // 收集成HashMap
        let names = ["sunface", "sunfei"];
        let ages = [18, 18];
        let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
        // zip是一个迭代器适配器，参数是另外一个迭代器，然后压缩到一起[(name1, age1), (name2, age2)]

        // 闭包作为适配器参数
    }

    // 7. 实现Iterator特征
    {
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Self {
                Self { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        // Counter实现了Iterator特征，可以当作一个迭代器使用
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);

        // Iterator特征中的其他方法，诸如map, zip这些都有默认实现，可以不要手动实现
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        // 形如 [1, 2, 3, 4, 5] 和 [2, 3, 4, 5] 的迭代器合并后
        // 新的迭代器形如 [(1, 2),(2, 3),(3, 4),(4, 5)]
        // map之后 [2, 6, 12, 20]
        // filter过滤之后[6, 12]
        assert_eq!(18, sum);

        // enumerator
        // v.iter() 创建迭代器，其次 调用 Iterator 特征上的方法 enumerate，该方法产生一个新的迭代器，其中每个元素均是元组 (索引，值)
        let v = vec![1u64, 2, 3, 4, 5, 6];
        for (i, v) in v.iter().enumerate() {
            println!("第{}个值是{}", i, v)
        }

        // 因为 enumerate 是迭代器适配器，因此我们可以对它返回的迭代器调用其它 Iterator 特征方法
        let val = v.iter()
        .enumerate()
        // 每两个元素删一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(idx, val)| val)
        // 累加
        .fold(0u64, |sum, acm| sum + acm);
    }
}
