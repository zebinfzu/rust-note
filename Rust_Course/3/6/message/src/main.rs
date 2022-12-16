#![allow(unused)]
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    // 1. rust创建一个消息通道，返回一个元组，(发送者,接收者)
    {
        // mpsc multiple producer, single consumer
        let (tx, rx) = mpsc::channel();
        // 创建消息，并发送消息
        thread::spawn(move || {
            // 发送一个数字1，send方法返回Result<T, E>，通过unwrap进行快速错误处理
            // 发送消息的值的类型是固定的，第一次发送，编译器就会按照这个值来推动这个发送器发送的数值类型
            tx.send(1).unwrap();
            // 下面错误将会报错，因为编译器自动推导传递的值i32类型，那么Option<i32>类型将产生不匹配错误
            // tx.send(Some(1)).unwrap();
        });
        // 在主线程中接收子线程发送的消息并输出
        // recv会阻塞当前线程，直到收到消息
        println!("receive {}", rx.recv().unwrap());
    }
    // 2. rust不阻塞的try_recv方法
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(1).unwrap();
        });
        // try_recv不会阻塞当前线程，返回值是一个Result，没有值就得到Err
        println!("receive {:?}", rx.try_recv());
    }

    // 3. 传输具有所有权的数据
    // 值的类型实现了Copy特征，就会复制一份
    // 没有实现则会把所有权给过去

    // 4. 使用for循环接收
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            // 发送一条消息，线程休眠1s
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        // 主线程循环阻塞的从rx迭代器中取到数据
        for received in rx {
            println!("Got: {}", received);
        }
    }

    // 5. 使用多发送者
    {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            tx.send(String::from("hi from raw tx")).unwrap();
        });
        thread::spawn(move || {
            tx1.send(String::from("hi from cloned tx")).unwrap();
        });
        // 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
        // 由于两个子线程谁先创建完成是未知的，因此哪条消息先发送也是未知的，最终主线程的输出顺序也不确定
        for received in rx {
            println!("Got: {}", received);
        }
    }

    // 6. 消息发送接收在通道当中是遵循FIFO原则的
    // 上面不确定顺序是由于线程创建顺序不确定导致的

    // 7. mpsc 在rust中其实是提供了异步和同步两种版本的
    // 之前使用的都是默认的异步通道，即发送完了不会阻塞当前线程，可以继续执行
    {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            println!("发送之前");
            tx.send(1).unwrap();
            println!("发送之后");
        });

        println!("睡眠之前");
        thread::sleep(Duration::from_secs(3));
        println!("睡眠之后");

        println!("receive {}", rx.recv().unwrap());
        handle.join().unwrap();
    }

    // 同步通道，发送之后阻塞当前线程，直到消息被接收才会解除阻塞
    {
        // bound指定消息队列中最大的无阻塞消息数n，当消息队列中等待接收的消息大于n将会阻塞线程
        let (tx, rx) = mpsc::sync_channel(0);

        let handle = thread::spawn(move || {
            println!("发送之前");
            tx.send(1).unwrap();
            println!("发送之后");
        });

        println!("睡眠之前");
        thread::sleep(Duration::from_secs(3));
        println!("睡眠之后");

        println!("receive {}", rx.recv().unwrap());
        handle.join().unwrap();
    }

    // 8. 当发送者和接收者都被drop之后，消息队列自动drop，rust中这是编译期就确定的

    // 9. 一个消息队列只能发送一种数据类型，可以通过枚举来实现发送不同的数据类型

    // 10. 因为drop的发生是自动的，所有要注意以下的情况
    {
        let (send, recv) = mpsc::channel();
        let num_threads = 3;
        // 子线程中的send都是clone的
        for i in 0..num_threads {
            let thread_send = send.clone();
            thread::spawn(move || {
                thread_send.send(i).unwrap();
                println!("thread {:?} finished", i);
            });
        }

        // 在这里drop send...
        // recv这个迭代器会等到send全部drop才会结束，因此原来的send这里就要手动drop，不然等到主线程结束才drop的话就会导致一直卡在下面这个循环
        drop(send);
        for x in recv {
            println!("Got: {}", x);
        }
        println!("finished iterating");
    }

    // 11. mpmc多发送者多接收者，使用第三方库crossbeam-channel或者flume
}
