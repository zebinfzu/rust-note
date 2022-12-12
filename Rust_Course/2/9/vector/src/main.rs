#[allow(unused)]
fn main() {
    // 1. rust的原始类型数组是静态长度的
    // 2. Vector是动态长度的数组
    // 3. 使用Vec<>来标注元素的类型，vector同样要求数组元素的类型是固定的
    let v: Vec<i32> = Vec::new();

    // 4. 使用数组来初始化一个Vector
    let v = Vec::from([1, 2, 3]);

    // 5. 使用vec!宏来初始化一个vector
    let v = vec![1, 2, 3];
    // 6. 增
    let mut v: Vec<i32> = vec![];
    {
        v.push(5);
    }
    // 7. Vector和元素共存亡，Vector被Drop之后内部元素也被删除
    // 8. 使用[index]读取，越界会导致Panic
    // 9. 使用get(index)读取，会返回Option值
    // 10. 借用数组元素的值会产生不可变引用，这时候使用push之类的方法会导致同时出现可变引用和不可变引用的问题

    // 11. 遍历迭代Vector
    {
        let v = vec![1, 2, 3];
        for i in &v {
            println!("{}", i);
        }
        // 迭代的时候更新数组元素
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 10;
        }
    }
    // 12. 存储不同类型的值
    {
        // 使用枚举
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let v = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];
        {
            // 利用特征对象
            trait IpAddr {
                fn display(&self);
            }

            struct V4(String);
            impl IpAddr for V4 {
                fn display(&self) {
                    println!("ipv4: {:?}", self.0)
                }
            }
            struct V6(String);
            impl IpAddr for V6 {
                fn display(&self) {
                    println!("ipv6: {:?}", self.0)
                }
            }

            let v: Vec<Box<dyn IpAddr>> = vec![
                Box::new(V4("127.0.0.1".to_string())),
                Box::new(V6("::1".to_string())),
            ];

            for ip in v {
                ip.display();
            }
        }
    }
}
