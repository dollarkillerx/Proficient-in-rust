use futures::executor::block_on;
use std::thread;
use std::time::Duration;

async fn hello_world() {
    println!("Hello World");
}

async fn learn_and_sing() {
    let _song = learn_song().await; // 等待 learn_song 运行结束
}

async fn learn_song() {
    thread::sleep(Duration::from_millis(5000));
    println!("aaaaa")
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = hello_world();

    futures::join!(f1,f2); // futures 等待这两个future运行结束
}

fn main() {
    block_on(async_main());
    println!("Hello, world!");
}
