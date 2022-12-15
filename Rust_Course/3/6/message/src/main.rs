#![allow(unused)]
use std::sync::mpsc;
use std::thread;
fn main() {
    // 1. rust创建一个消息通道，返回一个元组，(发送者,接收者)
    {
        // mpsc multiple producer, single consumer
        let (tx, rx) = mpsc::channel();
        // 创建消息，并发送消息
        thread::spawn(move || {
            // 发送一个数字1，send方法返回Result<T, E>，通过unwrap进行快速错误处理
            tx.send(1).unwrap();
            // 下面错误将会报错，因为编译器自动推导传递的值i32类型，那么Option<i32>类型将产生不匹配错误
            // tx.send(Some(1)).unwrap();
        });
        // 在主线程中接收子线程发送的消息并输出
        println!("receive {}", rx.recv().unwrap());
    }
    // 2. rust不阻塞的try_recv方法
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(1).unwrap();
        });
        println!("receive {:?}", rx.try_recv());
    }
}