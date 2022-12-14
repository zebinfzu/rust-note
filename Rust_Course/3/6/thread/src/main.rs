use std::{
    cell::{Cell, RefCell},
    sync::{Arc, Barrier},
    thread,
    time::Duration,
};

fn main() {
    // 多线程编程的风险
    // 1. 竞态条件(race conditions)，多个线程以非一致性的顺序同时访问数据资源
    // 2. 死锁(deadlocks)，两个线程都想要使用某一个资源，但是都在等待对方释放资源后才能使用，最后导致都无法进行
    // 3. 一些因为多线程导致的隐晦的BUG，难以复现和解决

    // 1. thread::spawn 可以创建线程
    {
        // 线程内部的代码使用闭包来执行
        // main线程一旦结束，程序立刻结束，因此要保持main线程存活，知道子线程完成自己的任务
        // 因此可以看到下面这个线程打不完1到10
        thread::spawn(|| {
            for i in 1..10 {
                println!("数字 {} 来自新线程!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..10 {
            println!("数字 {} 来自主线程!", i);
            thread::sleep(Duration::from_millis(1));
        }

        let handle = thread::spawn(|| {
            for i in 1..5 {
                println!("数字 {} 来自会阻塞主线程的新线程!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // join方法会block当前线程，直到子线程结束
        handle.join().unwrap();
    }

    // 2. 在线程闭包中使用move
    {
        // 由于Rust不知道新线程会活多久，所以没法使用闭包来引用原来线程中的代码
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }

    // 3. 线程如何结束
    // main线程是程序的主线程，一旦结束，程序随之结束，同时各个子线程也会被强行终止
    // 线程的代码执行完，线程就会自动结束。
    // 如果线程中的代码执行不完，分为两种情况

    // 4. 线程的任务是一个循环IO读取，任务流程类似：IO 阻塞，等待读取新的数据 -> 读到数据，处理完成 -> 继续阻塞等待 ··· -> 收到 socket 关闭的信号 -> 结束线程，在此过程中，绝大部分时间线程都处于阻塞的状态，因此虽然看上去是循环，CPU 占用其实很小，也是网络服务中最最常见的模型

    // 5. 线程的任务是一个循环，里面没有任何阻塞，包含休眠这一类的操作也无，此时CPU会不幸的被跑满，而且没有设置终止条件，该线程会一直跑满一个CPU核心，并且不会终止，直到main线程结束
    {
        // 创建线程A
        let new_thread = thread::spawn(move || {
            // 创建线程B
            thread::spawn(move || loop {
                println!("I am a new thread.");
            })
        });

        // 等待新线程创建线程执行完毕
        new_thread.join().unwrap();
        println!("Child thread is finish!");

        // 睡眠一段时间，看看子线程创建的线程是不是还在执行
        thread::sleep(Duration::from_millis(10));
    }

    // 6. 创建线程是有开销的，因此只有值得使用线程的任务才使用线程，免得开线程的消耗都比任务的消耗大了
    // 7. 创建多少线程合适：当任务是 CPU 密集型时一般最多和cpu核心数一样多。但是当你的任务大部分时间都处于阻塞状态时，就可以考虑增多线程数量，典型就是网络 IO 操作，但是事实上，对于这种网络 IO 情况，一般都不再使用多线程的方式了，毕竟操作系统的线程数是有限的，意味着并发数也很容易达到上限，而且过多的线程也会导致线程上下文切换的代价过大，使用 async/await 的 M:N 并发模型，就没有这个烦恼

    // 8. 线程屏障(Barrier)
    // Rust中，可以使用Barrier让多个线程都执行到某个点之后，才会继续一起往后执行
    {
        let mut handles = Vec::with_capacity(6);
        let barrier = Arc::new(Barrier::new(6));
        for _ in 0..6 {
            let b = barrier.clone();
            handles.push(thread::spawn(move || {
                println!("before wait");
                // 在线程打印出 before wait 后增加了一个屏障
                b.wait();
                // 目的就是等所有的线程都打印出before wait后，各个线程再继续执行
                println!("after wait");
            }))
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // 7. 线程局部变量：标准库thread_local
    {
        // 线程宏thread_local初始化线程局部变量
        thread_local! (static FOO: RefCell<u32> = RefCell::new(1));
        // 然后再线程的内部使用该变量的with方法获取变量的值
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });

        // 每个线程开始时都会拿到线程局部变量的FOO的初始值
        let t = thread::spawn(move || {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
        });

        // 等待线程完成
        t.join().unwrap();

        // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });

        // FOO 即是我们创建的线程局部变量
        // 每个新线程访问它时，都会用它的初始值作为开始，各个线程中FOO值彼此之间互不干扰
        // 使用static声明声明周期为'static

        // 可以注意到，线程中对FOO的使用是通过借用的方式，但是如果我们要每个线程独自获取拷贝，最后汇总就强人所难了

        // 结构体当中使用线程局部变量
        struct Foo;
        impl Foo {
            thread_local! {
                static FOO: RefCell<usize> = RefCell::new(0);
            }
        }

        Foo::FOO.with(|x| println!("{:?}", x));
    }

    // 8. 第三方库thread-local
    // 允许每个线程持有值的独立拷贝
    {
        use thread_local::ThreadLocal;

        let tls = Arc::new(ThreadLocal::new());

        // 创建多个线程
        for _ in 0..5 {
            let tls2 = tls.clone();
            thread::spawn(move || {
                // 将计数器+1
                let cell = tls2.get_or(|| Cell::new(0));
                cell.set(cell.get() + 1);
            })
            .join()
            .unwrap();
        }

        // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
        let tls = Arc::try_unwrap(tls).unwrap();
        let total = tls.into_iter().fold(0, |x, y| x + y.get());

        // 和为5
        assert_eq!(total, 5);
    }

    // 9. 使用条件变量(condition variables)控制线程的挂起和执行
    {
        use std::sync::{Mutex, Condvar};

        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        thread::spawn(move || {
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            println!("changing started");
            *started = true;
            cvar.notify_one();
        });

        let &(ref lock, ref cvar) = &*pair;

        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!("stared changed!");

        // main 线程首先进入 while 循环，调用 wait 方法挂起等待子线程的通知，并释放了锁 started
        // 子线程获取到锁，并将其修改为 true，然后调用条件变量的 notify_one 方法来通知主线程继续执行
    }

    // 10. 只会被调用一次的函数call_once 方法

    // 11. 总结
    // rust 的线程模型是和os线程 1:1，保证运行时尽可能小
    // thread::spawn创建线程，创建出来的线程没有执行顺序，代码逻辑千万不要依赖于线程间的执行顺序
    // main结束，所有子线程结束，如果希望子线程结束再结束main，需要使用创建线程时候返回的句柄的join方法
    // 线程执行的代码是传入一个闭包，但是这个闭包不能直接使用捕获的值，必须要使用move将变量的所有权给新线程才行
    
}
