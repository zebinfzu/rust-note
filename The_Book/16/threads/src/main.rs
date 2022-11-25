#![allow(unused)]
use std::thread;
use std::time::Duration;

fn main() {
    // 1. 注意一旦主线程结束，不管新的线程结束与否，新线程也会结束，因此可以看到新线程很可能没有打印到10
    /*
    {
        // 使用spawn方法创建一个一个新线程并传入一个闭包作为希望在新线程执行的代码
        thread::spawn(|| {
            for i in 1..10 {
                println!("这个数 {} 来自于新开的线程!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("这个数 {} 来自主线程!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    */

    // 2. 使用 join 等待所有线程结束
    /*
    {
        // handle会保存一个JoinHandle以保证该线程可以结束
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap(); // join方法一定会保证该线程执行完之后再执行后面的代码

        for i in 1..5 {
            println!("主线程等待到新线程执行完之后才会继续执行{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    */

    // 3. 线程和move闭包
    {
        // 在另外一个线程使用闭包捕获的数据需要使用move将所有权给过去
        let v = vec![1, 2, 3];
        // Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效，因此创建线程的时候就要把捕获数据的所有权给进去
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        

        handle.join().unwrap();
    }
}
