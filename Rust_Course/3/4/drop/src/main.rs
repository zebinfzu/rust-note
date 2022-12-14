#[allow(unused)]
fn main() {
    // 1. Drop特征，在内存回收的时候执行
    {
        struct HasDrop1;
        struct HasDrop2;
        impl Drop for HasDrop1 {
            fn drop(&mut self) {
                println!("Dropping HasDrop1!");
            }
        }
        impl Drop for HasDrop2 {
            fn drop(&mut self) {
                println!("Dropping HasDrop2!");
            }
        }

        struct HasTwoDrops {
            one: HasDrop1,
            two: HasDrop2,
        }
        impl Drop for HasTwoDrops {
            fn drop(&mut self) {
                println!("Dropping HasTwoDrops!");
            }
        }

        struct Foo;

        impl Drop for Foo {
            fn drop(&mut self) {
                println!("Dropping Foo!")
            }
        }

        let _x = HasTwoDrops {
            two: HasDrop2,
            one: HasDrop1,
        };
        let _foo = Foo;
        println!("Running!");
    } // drop _x _foo

    // 上面例子的输出
    // Running!
    // Dropping Foo!
    // Dropping HasTwoDrops!
    // Dropping HasDrop1!
    // Dropping HasDrop2!

    // 观察Drop特征的输出顺序
    // 变量级别: 按照逆序，_x 创建在 _foo 之前，因此 _x 在 _foo 之后被drop
    // 结构体内部: 按照字段定义的顺序
    // rust几乎为所有类型都提供了Drop特征的默认实现，因此就算你不手动为结构体实现 Drop，它依然会调用默认实现的 drop 函数，同时再调用每个字段的 drop 方法

    // 2. 手动回收
    // 如管理锁的场景，希望提取释放锁，让其他线程可以获得锁，Drop::drop 只是借用了目标值的可变引用，所以，就算你提前调用了 drop，后面的代码依然可以使用目标值，但是这就会访问一个并不存在的值，非常不安全
    // 所以rust禁止手动调用drop方法
    {
        #[derive(Debug)]
        struct Foo;

        impl Drop for Foo {
            fn drop(&mut self) {
                println!("Dropping Foo!")
            }
        }

        let foo = Foo;
        // foo.drop(); 编译器报错，禁止手动调用drop，因为调用了drop到作用域结束还是会自己drop，这样还是个二次释放的问题
        println!("Running!:{:?}", foo);

        // 应该使用std::mem::drop函数，这个函数拿走的是数据的所有权，因此在作用域结束的时候也不会发生二次释放的问题
        drop(foo);
    }

    // 3. 函数drop的使用场景
    // 回收内存资源
    // 执行一些收尾工作
    // 确实有极少数情况，需要自己来回收资源的
    // 例如文件描述符、网络 socket 等，当这些值超出作用域不再使用时，就需要进行关闭以释放相关的资源，在这些情况下，就需要使用者自己来解决 Drop 的问题

    // 4. Drop特征和Copy特征互斥
}
