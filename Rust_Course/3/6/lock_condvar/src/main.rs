use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

// 用于rust的惰性计算宏
use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

#[allow(unused)]
fn main() {
    // 在多线程编程中，同步性极其的重要，当你需要同时访问一个资源、控制不同线程的执行次序时，都需要使用到同步性

    // 消息传递就是同步性的一种实现方式，消息传递类似一个单所有权的系统：一个值同时只能有一个所有者，如果另一个线程需要该值的所有权，需要将所有权通过消息传递进行转移
    // 需要可靠和简单的(简单不等于简洁)实现时
    // 需要模拟现实世界，例如用消息去通知某个目标执行相应的操作时
    // 需要一个任务处理流水线(管道)时，等等

    // 而共享内存：锁+原子操作 需要简洁的实现以及更高的性能时
    // 类似于一个多所有权的系统：多个线程可以同时访问同一个值

    // 1. 互斥锁mutex: 同一时间只能有一个线程访问A的数据，其他线程需要等待A的锁被释放
    // 单线程中使用互斥锁
    {
        let m = Mutex::new(5);

        {
            // 获取锁，然后deref为`m`引用
            // lock返回的是Result
            // m.lock()向m申请一个锁, 该方法会阻塞当前线程，直到获取到锁
            // m.lock()方法也有可能报错，例如当前正在持有锁的线程panic
            // Mutex<T>是一个智能指针，调用.lock方法返回的是一个智能指针MutexGuard<T>
            // MutexGuard实现了Deref特征，会被自动解引用后获得一个引用类型，该引用指向Mutex内部的数据
            // 还实现了drop，在离开作用域的时候drop并释放锁
            let mut num = m.lock().unwrap();
            *num = 6;
            // 锁会自动drop
        }
        println!("m = {:?}", m);
    }

    // 导致死锁的情况
    {
        let m = Mutex::new(5);

        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
        drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
                   // 这里不drop的话就会导致下一行想要继续获得锁，但是第一个锁又需要等到主线程结束才会自动drop，从而导致死锁
        let mut num1 = m.lock().unwrap();
        *num1 = 7;
        // drop(num1); // 手动 drop num1 ，观察打印结果的不同

        println!("m = {:?}", m);
    }

    // 多线程使用mutex
    {
        // 利用线程安全的引用计数Arc来保存锁
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for idx in 0..10 {
            let counter = counter.clone();
            let handle = thread::spawn(move || {
                println!("{idx} child thread");
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        // 阻塞到新开的所有线程的执行完
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }

    // 2. Mutex同样支持内部可变，因此结合Arc来使用，就可以实现多线程的内部可变

    // 3. 死锁
    // 单线程死锁
    {
        let data = Mutex::new(0);
        let d1 = data.lock().unwrap();
        // let d2 = data.lock().unwrap();
    } // d1, d2 都在这里自动drop，因此d2要lock的时候会阻塞当前线程，但是d1也要等到当前块级作用域结束才drop，就导致了死锁

    // 多线程死锁
    // 拥有两个锁，且两个线程各自使用了其中一个锁，然后试图去访问另外一个，就可能发生死锁
    {
        // 存放子线程的句柄
        let mut children = vec![];
        for i_thread in 0..2 {
            children.push(thread::spawn(move || {
                for _ in 0..1 {
                    // 线程1
                    if i_thread % 2 == 0 {
                        // 锁住MUTEX1
                        let guard = MUTEX1.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);
                        // 当前线程睡眠一小会，等待线程2锁住MUTEX2
                        thread::sleep(Duration::from_millis(10));

                        // 去锁MUTEX2 // 如果这时候线程2锁住的MUTEX2，线程1就会block，一直等待线程2将MUTEX2释放
                        let guard = MUTEX2.lock().unwrap();
                    } else {
                        // 锁住MUTEX2
                        let _guard = MUTEX2.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX2，准备去锁MUTEX1", i_thread);
                        // 如果此时线程1锁住了MUTEX1并在等待线程2释放MUTEX1，就会导致死锁
                        // 但是如果线程2先执行，MUTEX1没有被锁住，就不会导致死锁
                        let _guard = MUTEX1.lock().unwrap();
                    }
                }
            }));
        }

        // 等子线程完成
        for child in children {
            let _ = child.join();
        }
        // 注意这里说的是死锁不一定发生，因为子线程的执行循序和执行速度是不确定，因此也无法确定哪个线程中的锁先被执行，因此也无法确定两个线程对锁的具体使用循序
        // 线程 1 锁住了MUTEX1并且线程2锁住了MUTEX2，然后线程 1 试图去访问MUTEX2，同时线程2试图去访问MUTEX1，就会死锁
        // 因为线程 2 需要等待线程 1 释放MUTEX1后，才会释放MUTEX2
        // 线程 1 需要等待线程 2 释放MUTEX2后才能释放MUTEX1，这种情况造成了两个线程都无法释放对方需要的锁，最终死锁
        println!("死锁没有发生");
    }

    // 4. try_lock，尝试获取一个锁，不会发生阻塞
    {
        // 存放子线程的句柄
        let mut children = vec![];
        for i_thread in 0..2 {
            children.push(thread::spawn(move || {
                for _ in 0..1 {
                    // 线程1
                    if i_thread % 2 == 0 {
                        // 锁住MUTEX1
                        let guard = MUTEX1.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                        // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                        thread::sleep(Duration::from_millis(10));

                        // 去锁MUTEX2
                        let guard = MUTEX2.try_lock();
                        println!("线程1获取MUTEX2锁的结果: {:?}", guard);
                    // 线程2
                    } else {
                        // 锁住MUTEX2
                        let _guard = MUTEX2.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                        thread::sleep(Duration::from_millis(10));
                        let guard = MUTEX1.try_lock();
                        println!("线程2获取MUTEX1锁的结果: {:?}", guard);
                    }
                }
            }));
        }

        // 等子线程完成
        for child in children {
            let _ = child.join();
        }

        println!("死锁没有发生");
    }

    // 5. 读写锁RwLock
    // Mutex每次读写都会进行加锁，但是需要大量并发读的情况下，Mutex就无法满足需求，此时可以使用RwLock
    {
        use std::sync::RwLock;

        let lock = RwLock::new(5);

        // 同一时间允许多个读
        {
            let r1 = lock.read().unwrap();
            let r2 = lock.read().unwrap();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        } // 读锁在此drop

        // 同一时间只允许一个写
        {
            let mut w = lock.write().unwrap();
            *w += 1;
            assert_eq!(*w, 6);
            // 以下代码会panic，因为读和写不允许同时存在
            // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
            // let r1 = lock.read();
            // println!("{:?}",r1);
        }

        // RwLock在使用上和Mutex区别不大，需要注意的是，当读写同时发生时，程序会直接panic(本例是单线程，实际上多个线程中也是如此)，因为会发生死锁

        // 可以使用try_write和try_read来尝试进行一次写/读，若失败则返回错误

        // 同时允许多个读，但最多只能有一个写
        // 读和写不能同时存在
        // 读可以使用read、try_read，写write、try_write, 在实际项目中，try_xxx会安全的多
    }

    // 6. 选择互斥锁还是读写锁
    // 追求高并发读取时，使用RwLock，因为Mutex一次只允许一个线程去读取
    // 如果要保证写操作的成功性，使用Mutex
    // 不知道哪个合适，统一使用Mutex
    // 一个常见的、错误的使用RwLock的场景就是使用HashMap进行简单读写，因为HashMap的读和写都非常快，RwLock的复杂实现和相对低的性能反而会导致整体性能的降低，因此一般来说更适合使用Mutex

    // 7. 三方库实现的锁，性能有些比标准库的更好
    // parking_lot 功能多，稳定
    // spin 性能更强

    // 8. 用条件变量(condition variable)控制线程的同步
    // 用来解决资源访问顺序的问题
    // 经常和Mutex一起使用，控制线程挂起和继续执行
    {
        // 创建线程安全的引用计数来使用互斥锁
        let flag = Arc::new(Mutex::new(false));
        // 创建线程安全的引用计数来使用条件变量
        let cond = Arc::new(Condvar::new());
        let cflag = flag.clone();
        let ccond = cond.clone();
        let hdl = thread::spawn(move || {
            let mut m = { *cflag.lock().unwrap() };
            let mut counter = 0;
            while counter < 3 {
                while !m {
                    m = *ccond.wait(cflag.lock().unwrap()).unwrap();
                }

                {
                    m = false;
                    *cflag.lock().unwrap() = false;
                }

                counter += 1;
                println!("inner counter: {}", counter);
            }
        });

        let mut counter = 0;
        loop {
            thread::sleep(Duration::from_millis(1000));
            *flag.lock().unwrap() = true;
            counter += 1;
            if counter > 3 {
                break;
            }
            println!("outside counter: {}", counter);
            cond.notify_one();
        }
        hdl.join().unwrap();
        println!("{:?}", flag);
        // 例子中通过主线程来触发子线程实现交替打印输出
    }

    // 9. 信号量(Semaphore)
    
}
