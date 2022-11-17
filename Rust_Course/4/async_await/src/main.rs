use futures::executor::block_on;
use async_std::task::sleep;
use std::time::Duration;
async fn do_something() {
    // 在异步函数中调用另外一个异步函数，等待执行完成之后再继续执行
    do_anything().await;
    println!("go go go!");
}

async fn do_anything() {
    println!("les les les");
}

fn main() {
    let future = do_something();
    // 执行future
    block_on(future);
    struct Song {
        author: String,
        name: String,
    }
    // 使用block_on，等到阻塞执行完了继续执行
    {
        async fn learn_song() -> Song {
            Song {
                author: "周杰伦".to_string(),
                name: String::from(" 《菊花台》 "),
            }
        }
        async fn sing_song(song: Song) {
            println!(
                "给大家献上一首{}的{} ~ {}",
                song.author, song.name, "菊花残，满地伤~ ~"
            );
        }
        async fn dance() {
            println!("唱到情深处，身体不由自主的动了起来~~");
        }
        let song = block_on(learn_song());
        block_on(sing_song(song));
        block_on(dance());
    }
    // 需要通过连续三次阻塞去等待三个任务的完成，一次只能做一件事，实际上我们完全可以载歌载舞啊
    // 使用join同时执行多个任务
    {
        async fn learn_song() -> Song {
            Song {
                author: "曲婉婷".to_string(),
                name: String::from("《我的歌声里》"),
            }
        }

        async fn sing_song(song: Song) {
            println!(
                "给大家献上一首{}的{} ~ {}",
                song.author, song.name, "你存在我深深的脑海里~ ~"
            );
        }

        async fn dance() {
            println!("唱到情深处，身体不由自主的动了起来~ ~");
        }
        async fn learn_and_sing() {
            // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
            let song = learn_song().await;
            // sleep 一会，这样就可以看出来异步
            sleep(Duration::from_secs(5)).await;
            // 唱歌必须要在学歌之后
            sing_song(song).await;
        }
        async fn async_main() {
            let f1 = learn_and_sing();
            let f2 = dance();
            // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
            // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
            futures::join!(f1, f2);
        }
        block_on(async_main());
    }
}
