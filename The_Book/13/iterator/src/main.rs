#![allow(unused)]

fn main() {
    // 1. 创建一个闭包
    {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();
        // 使用in运算符遍历一个迭代器
        for v in v1_iter {
            println!("Got: {}", v);
        }
    }
    // 2. 通过迭代器的next方法，挨个访问元素，最后会到None
    {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // 3. 迭代适配器
    {
        let v1 = vec![1, 2, 3];
        // 迭代适配器返回一个新的迭代器，比如map方法
        // let v2 = v1.iter().map(|x| x + 1); 警告指定的闭包没被调用过，说明迭代适配器是惰性的
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect将迭代器元素收集起来返回到vector中
    }

    // 4. 迭代适配器和捕获闭包环境结合的常见用法
    {
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }

        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    // 5. 实现Iterator trait来自定义迭代器
    {
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        // 测试next方法
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}
