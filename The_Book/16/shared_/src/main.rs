#![allow(unused)]
fn main() {
    // 回顾不要通过共享内存来通讯
    // 任何编程语言中的通道都类似于单所有权，因为一旦将一个值传送到通道中，将无法再使用这个值
    // 共享内存类似于多所有权：多个线程可以同时访问相同的内存位置

    // 互斥器(mutex) 是 mutual exclusion的缩写，就是说任意时刻，只允许一个线程访问这些数据
    // 为了访问互斥器中的数据，线程需要首先通过获取互斥器的锁(lock)来表示其想要访问数据

    // 锁作为互斥器一部分的数据结构，它记录谁有数据的排他访问权

    // 因此可以描述互斥器为通过锁系统保护其数据

    // Mutex的API

    // 1. 单线程上下文使用互斥器开始
    use std::sync::Mutex;
    {
        // 使用关联函数创造一个Mutex
        let m = Mutex::new(5);
        {
            // 使用lock方法获取锁，以访问锁内的数据
            // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败，所以使用unwrap并在这种情况下panic
            let mut num = m.lock().unwrap();
            // 一旦获取了锁，这里可以将num视为锁内数据的可变引用
            // 类型系统确保了我们在使用 m 中的值之前获取锁
            // 也就是说Mutex<T> 是一个智能指针
            // 更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针
            // 这个智能指针实现了 Deref 来指向其内部数据
            *num = 6;
        } // 其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
          // 丢弃了锁之后，可以打印出互斥器的值
        println!("m = {:?}", m);
    }

    // 2. 线程之间共享Mutex
    use std::rc::Rc;
    use std::thread;
    /*
    {
        // 因为我们要在多个线程之间使用同一个Mutex，因此需要一个可以有多个所有权的值
        // 如果Mutex在创建线程的时候是单所有权的，那么move了之后其他线程就不能使用了
        // 因此我们先想到了Rc引用计数
        // 但是编译还是会报错，编译器会告诉我们在不同线程之间使用Rc是不安全的
        // 当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断
        // 因此计数错误的时候很可能会出现一些逆天buf，比如循环引用
        let counter = Rc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Rc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
    */

    // 线程间安全的原子引用计数Arc<T> A表示原子性 atomic
    use std::sync::Arc;
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    // 我们声明的counter明明是不可以改变的，但是我们为什么又可以改变内部的值？
    // 这就意味着Mutex和RefCell类似的为我们提供了内部可变性
    // 因此可以认为Arc(Mutex)类似于Rc(RefCell)
    // 因此Arc(Mutex)一样有可能搞出来循环引用这种逆天bug
}
