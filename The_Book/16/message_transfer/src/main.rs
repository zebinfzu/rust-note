use std::thread;
use std::time::Duration;
fn main() {
    // 目前流行的确保安全并发的方式是消息传递
    // 不要通过共享内存来通讯，而是通过通讯来共享内存

    // Rust 中一个实现消息传递并发的主要工具是 通道（channel）
    use std::sync::mpsc;

    // 创建一个通道但没做，这样过不了编译，因为不知道要传递的是什么类型的消息
    // 这里使用的mpsc(multiple producer, single consumer)多个生产者，一个消费者
    // tx作为发送者，rx作为接收者
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // tx移动到一个新的线程，并发送一个'hi'
        tx.send(val).unwrap();
        // 发送之后不允许再使用该值val
        // 一旦将值发送到另一个线程后，那个线程可能会在我们再次使用它之前就将其修改或者丢弃
    });

    // 在主线程接收到通道发送的值，类似在河的下游把上游的值捞起来
    let received = rx.recv().unwrap(); // recv是会阻塞住当前的线程的，一直等到接收到
    println!("Got: {}", received);

    // 发送多个值并等待接收者接收
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // 不再显示的调用recv函数，而是把rx当作一个迭代器，对于每一个接收到的值，将其打印，通道被关闭的时候，迭代器也会结束
        for received in rx {
            println!("Got: {}", received);
        }

        println!("------");
    }
}
